use {
    anchor_lang::{solana_program::instruction::Instruction, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{Message, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

#[test]
fn test_initialize() {
    let program_id = test_project::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/test_project.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 1_000_000_000).unwrap();

    // 1. 새로운 카운터 계정을 위한 임시 열쇠쌍(Keypair)을 새로 생성합니다.
    let counter_keypair = Keypair::new();

    let instruction = Instruction::new_with_bytes(
        program_id,
        &test_project::instruction::Initialize {}.data(),
        test_project::accounts::Initialize {
            counter: counter_keypair.pubkey(), // 새로 만들 카운터 방의 주소
            user: payer.pubkey(),              // 돈을 지불할 내 지갑 주소
            system_program: anchor_lang::system_program::ID, // 솔라나 시스템 프로그램 주소
        }
        .to_account_metas(None),
    );

    let blockhash = svm.latest_blockhash();
    let msg = Message::new_with_blockhash(&[instruction], Some(&payer.pubkey()), &blockhash);
    let tx =
        VersionedTransaction::try_new(VersionedMessage::Legacy(msg), &[&payer, &counter_keypair])
            .unwrap();

    let res = svm.send_transaction(tx);
    assert!(res.is_ok());
}
