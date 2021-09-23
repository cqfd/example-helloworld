use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingAccount {
    /// number of greetings
    pub counter: u32,
    pub check: u32,
}

// Declare and export the program's entrypoint
entrypoint!(process_instruction);

// Program entrypoint's implementation
pub fn process_instruction(
    program_id: &Pubkey, // Public key of the account the hello world program was loaded into
    accounts: &[AccountInfo], // The account to say hello to
    instruction_data: &[u8], // Ignored, all helloworld instructions are hellos
) -> ProgramResult {
    msg!("Hello World Rust program entrypoint");

    // Iterating accounts is safer then indexing
    let accounts_iter = &mut accounts.iter();

    // Get the account to say hello to
    let account = next_account_info(accounts_iter)?;

    // The account must be owned by the program in order to modify its data
    if account.owner != program_id {
        msg!("Greeted account does not have the correct program id");
        return Err(ProgramError::IncorrectProgramId);
    }

    // Increment and store the number of times the account has been greeted
    let mut greeting_account = GreetingAccount::try_from_slice(&account.data.borrow())?;
    msg!("instruction_data = {:?}", instruction_data);
    greeting_account.counter = u32::try_from_slice(instruction_data)?;
    greeting_account.check = greeting_account.counter % 2;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Greeted {} time(s)!", greeting_account.counter);

    Ok(())
}

// Sanity tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::clock::Epoch;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; 4 + 4];
        let owner = Pubkey::default();
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let instruction_data: Vec<u8> = vec![1, 2, 3, 4];

        let accounts = vec![account];

        let ga = GreetingAccount::try_from_slice(&accounts[0].data.borrow()).unwrap();
        assert_eq!(ga.counter, 0);
        assert_eq!(ga.check, 0);

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        let ga = GreetingAccount::try_from_slice(&accounts[0].data.borrow()).unwrap();
        assert_eq!(
            ga.counter,
            1 + 2 * 256 + 3 * 256 * 256 + 4 * 256 * 256 * 256
        );
        assert_eq!(ga.check, 1); // ^ that number is odd

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();
        let ga = GreetingAccount::try_from_slice(&accounts[0].data.borrow()).unwrap();
        assert_eq!(
            ga.counter,
            1 + 2 * 256 + 3 * 256 * 256 + 4 * 256 * 256 * 256
        );
        assert_eq!(ga.check, 1); // ^ that number is odd
    }
}
