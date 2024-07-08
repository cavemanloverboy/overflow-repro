# `overflow-repro`

A minimal reproducer of a solana program that overflows stack and corrupts data on another frame.



### Program
```rust
#[no_mangle]
pub unsafe extern "C" fn entrypoint(mut input: *mut u8) -> u32 {
    // In this simple repoducer, there are no input accounts
    input = input.add(size_of::<u64>());

    // Instruction data length should be 3
    assert_eq!(*input.cast::<u64>(), 3, "unexpected input length");
    input = input.add(size_of::<u64>());

    // Get instruction data
    let data = core::slice::from_raw_parts(input, 3);

    // Copy data to stack and corrupt
    let data_on_stack = [data[0], data[1], data[2]];
    corrupt();

    // Log and set as return
    log!("data {:?}", &data_on_stack);
    set_return(&data_on_stack);

    0
}

#[inline(never)]
fn corrupt() {
    let mut oops = [0_u8; 8192];
    oops[0..8192].fill(5);
    std::hint::black_box(oops);
}
```


### Client
```rust
#[tokio::test]
async fn reproduce_corruption_via_overflow() {
    let mut pt = ProgramTest::default();
    pt.prefer_bpf(true);
    pt.add_program("overflow_repro", REPRO_ID, None);
    let mut ctx = pt.start_with_context().await;

    let input_data = [1, 2, 3];
    let invoke_ix = Instruction {
        program_id: REPRO_ID,
        accounts: vec![],
        data: input_data.to_vec(),
    };
    let invoke_transaction = Transaction::new_signed_with_payer(
        &[invoke_ix],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer],
        ctx.last_blockhash,
    );

    let Ok(BanksTransactionResultWithMetadata {
        metadata:
            Some(TransactionMetadata {
                return_data:
                    Some(TransactionReturnData {
                        data: returned_data,
                        ..
                    }),
                ..
            }),
        ..
    }) = ctx
        .banks_client
        .process_transaction_with_metadata(invoke_transaction)
        .await
    else {
        panic!("failed invocation");
    };

    // This should have just been the input but it was corrupted!
    assert!(returned_data != input_data);
}
```

To reproduce, run `cargo-test-sbf` using a newer 1.18 toolchain (e.g. 1.18.17).