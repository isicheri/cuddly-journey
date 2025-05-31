
use solana_program::{
    account_info::AccountInfo,entrypoint,entrypoint::ProgramResult,msg,pubkey::Pubkey,
};

entrypoint!(process_instruction);


pub fn process_instruction(
        _program_id: &Pubkey, _accounts: &[AccountInfo],
        _instruction_data: &[u8],
) -> ProgramResult {
   msg!("hello world");
   Ok(())
}




#[cfg(test)]
mod test {
    use solana_program_test::*;
    use solana_sdk::{
        instruction::Instruction, pubkey::Pubkey, signature::Signer, transaction::Transaction
    };


    #[tokio::test]
    async fn t_hello_world() {
        let program_id = Pubkey::new_unique();
        let mut program_test = ProgramTest::default();
        program_test.add_program("hello-smart-contract", program_id, None);
        let (mut banks_client,payer,recent_block_hash) = program_test.start().await;

        //create the instruction
        let instruction = Instruction {
            program_id,
            accounts: vec![],
            data: vec![]
        };

        //create the transaction
        let mut transaction = Transaction::new_with_payer(&[instruction],Some(&payer.pubkey()));

        //sign the transaction 
        transaction.sign(&[&payer], recent_block_hash);

        let transaction_result = banks_client.process_transaction(transaction).await;

        assert!(transaction_result.is_ok());
    }

}