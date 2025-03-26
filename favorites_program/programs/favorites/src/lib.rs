use anchor_lang::prelude::*;

declare_id!("2C8rYpRuNHHeYbzixiGFEUWHie3U8cqDdRVini373oLL");

pub const ANCHOR_DESCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favourites {
    use super::*;

    pub fn set_favourites(
        ctx: Context<SetFavourites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        msg!("Greetings from {}!!", ctx.program_id);
        let signer_public_key = ctx.accounts.signer.key();
        msg!("User {}!!", signer_public_key);
        msg!(
            "User {} favourite color is {}, and their hobbies are {:?}",
            signer_public_key,
            color,
            hobbies
        );

        ctx.accounts.favourites.set_inner(Favourites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favourites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavourites<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_DESCRIMINATOR_SIZE + Favourites::INIT_SPACE,
        seeds = [b"favourites", signer.key().as_ref()],
        bump
    )]
    pub favourites: Account<'info, Favourites>,

    pub system_program: Program<'info, System>,
}

