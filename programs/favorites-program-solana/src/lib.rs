use anchor_lang::prelude::*;

declare_id!("6brs1kNvGxFDnvrGyqQHrJdwbuGL3hup4vodJUgKnED4");

pub const ANCHOR_DISCRIMINATOR_SIZE:usize = 8;

#[program]
pub mod favorites_program_solana {

    use super::*;

    pub fn set_favorites(
        ctx: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>
    ) -> Result<()> {
        msg!("Greetings from {}", ctx.program_id);
        ctx.accounts.favorites.set_inner(Favourites{
            number,
            color,
            hobbies
        });
        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favourites {

    pub number:u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5,50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info>{

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favourites::INIT_SPACE,
        seeds = [b"favorites", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>
}