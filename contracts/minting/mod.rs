
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    program::invoke,
    pubkey::Pubkey,
};
use spl_token::instruction as token_instruction;

pub fn mint_tokens(
    accounts: &[AccountInfo],
    mint_amount: u64, // Amount to mint
) -> solana_program::entrypoint::ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let mint_account = next_account_info(account_info_iter)?;
    let destination_account = next_account_info(account_info_iter)?;
    let authority_account = next_account_info(account_info_iter)?;

    let mint_instruction = token_instruction::mint_to(
        &spl_token::id(),
        &mint_account.key,
        &destination_account.key,
        &authority_account.key,
        &[],
        mint_amount,
    )?;

    invoke(
        &mint_instruction,
        &[mint_account.clone(), destination_account.clone(), authority_account.clone()],
    )
}
