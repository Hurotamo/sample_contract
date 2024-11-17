use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program::invoke,
    pubkey::Pubkey,
};
use spl_token::instruction as token_instruction;

pub fn burn_tokens(
    accounts: &[AccountInfo],
    burn_amount: u64, // Amount to burn
) -> solana_program::entrypoint::ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let burn_account = next_account_info(account_info_iter)?;
    let token_account = next_account_info(account_info_iter)?;
    let authority_account = next_account_info(account_info_iter)?;

    let burn_instruction = token_instruction::burn(
        &spl_token::id(),
        &token_account.key,
        &burn_account.key,
        &authority_account.key,
        &[],
        burn_amount,
    )?;

    invoke(
        &burn_instruction,
        &[burn_account.clone(), token_account.clone(), authority_account.clone()],
    )
}

