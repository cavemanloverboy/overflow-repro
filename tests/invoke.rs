use solana_banks_interface::TransactionMetadata;
use solana_program_test::{BanksTransactionResultWithMetadata, ProgramTest};
use solana_sdk::{
    instruction::Instruction, pubkey::Pubkey, signer::Signer, transaction::Transaction,
    transaction_context::TransactionReturnData,
};

const REPRO_ID: Pubkey = Pubkey::new_from_array([69; 32]);
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
