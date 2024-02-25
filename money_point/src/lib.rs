use solana_program::{
    account_info::{ AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

// Declare and export the program's entry point
entrypoint!(process_instruction);

// Program entry point's implementation
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Parse the instruction data to determine the operation and operands
    let instruction_str = match instruction_data.get(0) {
        Some(&instruction) => instruction,
        None => {
            msg!("No instruction provided");
            return Ok(());
        }
    };

    let operands = match instruction_data.get(1..) {
        Some(slice) => slice,
        None => {
            msg!("No operands provided");
            return Ok(());
        }
    };

    // Ensure that the accounts provided are sufficient for the operation
    if accounts.len() < 1 {
        msg!("Insufficient accounts provided");
        return Ok(());
    }

    // Retrieve the account that will store the result
    let result_account = &accounts[0];

    // Perform the requested operation
    let result = match instruction_str {
        b'+' => {
            if operands.len() < 2 {
                msg!("Addition requires at least two operands");
                return Ok(());
            }
            let operand1 = operands[0] as u64;
            let operand2 = operands[1] as u64;
            operand1 + operand2
        }
        b'-' => {
            if operands.len() < 2 {
                msg!("Subtraction requires at least two operands");
                return Ok(());
            }
            let operand1 = operands[0] as u64;
            let operand2 = operands[1] as u64;
            operand1 - operand2
        }
        b'*' => {
            if operands.len() < 2 {
                msg!("Multiplication requires at least two operands");
                return Ok(());
            }
            let operand1 = operands[0] as u64;
            let operand2 = operands[1] as u64;
            operand1 * operand2
        }
        b'/' => {
            if operands.len() < 2 || operands[1] == 0 {
                msg!("Division requires at least two operands, and the second operand must not be zero");
                return Ok(());
            }
            let operand1 = operands[0] as u64;
            let operand2 = operands[1] as u64;
            operand1 / operand2
        }
        _ => {
            msg!("Unsupported operation");
            return Ok(());
        }
    };

    // Write the result to the result account
    let mut result_data = result.to_le_bytes();
    result_account
        .try_borrow_mut_data()
        .map(|mut data| {
            data[..8].copy_from_slice(&result_data);
        })
        .map_err(|_| {
            msg!("Failed to borrow result account data");
            solana_program::program_error::ProgramError::AccountBorrowFailed
        })?;

    msg!("Result: {}", result);

    Ok(())
}
