use anchor_lang::prelude::*;

#[account]
pub struct ZerisState {
    pub last_proof_hash: [u8; 32],
    pub merkle_root: [u8; 32],
    pub proof_count: u64,
    pub authority: Pubkey,
    pub bump: u8,
}

impl ZerisState {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 32 + 1; // discriminator + fields
}

#[derive(Accounts)]
#[instruction(proof_bytes: Vec<u8>, public_signals: Vec<u8>)]
pub struct SubmitProof<'info> {
    #[account(
        init_if_needed,
        payer = signer,
        space = ZerisState::LEN,
        seeds = [b"zeris-state"],
        bump
    )]
    pub zeris_state: Account<'info, ZerisState>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VerifyProof<'info> {
    #[account(
        mut,
        seeds = [b"zeris-state"],
        bump = zeris_state.bump
    )]
    pub zeris_state: Account<'info, ZerisState>,
}

#[derive(Accounts)]
pub struct ValidateMerkleRoot<'info> {
    #[account(
        seeds = [b"zeris-state"],
        bump = zeris_state.bump
    )]
    pub zeris_state: Account<'info, ZerisState>,
}