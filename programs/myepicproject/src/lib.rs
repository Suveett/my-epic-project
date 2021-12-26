use anchor_lang::prelude::*;

declare_id!("FqTvUDRzDvUuMVJFcsZXrxbbtHaNbzAepEJD7j4Y4uQ5");

#[program]
pub mod myepicproject {
    use super::*;
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
        //Get a referrence to the account.
        let base_account = &mut ctx.accounts.base_account;
        //Initialize total Gifs.
        base_account.total_gifs = 0;
        base_account.gif_list = []
        Ok(())
    }

    pub fn add_gifs(ctx : Context<AddGifs>, gif_link : String) -> ProgramResult {
        //Get a referrence to the account.
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        //Lets build an ItemStruct here. 
        let item = ItemStruct {
            gif_link : gif_link.to_string(),
            user_address : *user.to_account_info().key,
        };

        //Now lets push this gif_link to gif_list
        base_account.gif_list.push(item);

        //Increment the Gif Count. 
        base_account.total_gifs += 1;    

        Ok(())   
    }
    //This below code works if the source code(i.e program) is owned by {SystemProgram} = solana_program BPFUpgradeable111111111, which is true here.
    /*pub fn send_sol(ctx: Context<SendSol>, amount : u64) -> ProgramResult {
       
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.base_account.key(),
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.base_account.to_account_info(),
            ],
        );
        Ok(())
        
    }*/

    //This below code works if the source code is owned by the programId itself.
    /*pub fn send_sol(ctx: Context<SendSol>, amount : u64) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;
        **user.to_account_info().try_borrow_mut_lamports()? -= amount;
        **base_account.to_account_info().try_borrow_mut_lamports()? += amount;
        Ok(())
    }*/
    
    /*pub fn send_sol(ctx: Context<SendSol>, amount : u64) -> ProgramResult {
       
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &user.to_account_info().key,
            &base_account.to_account_info().key,
            amount,
        );
        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                user.to_account_info(),
                base_account.to_account_info(),
            ],
        );
        Ok(())
        
    }*/

    //Attach certain variables to the StartStuffOff Context. 
    #[derive(Accounts)]
    pub struct StartStuffOff<'info> {
        
        #[account(init, payer = user, space = 9000)]
        pub base_account : Account<'info, BaseAccount>,
        #[account(mut)]
        pub user : Signer<'info>,
        pub system_program : Program<'info, System>,
        
    }

    //Create a custom struct for us to work with. 
    #[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
    pub struct ItemStruct {
        pub gif_link : String,
        pub user_address : Pubkey,
    }

    //Specify the Data we wanty to add in the AddGif Context.
    #[derive(Accounts)]
    pub struct AddGifs<'info> {
        #[account(mut)]
        pub base_account : Account<'info, BaseAccount>,
        #[account(mut)]
        pub user : Signer<'info>,

    }

    //Specify the to and from accounts we want in SendSol.
    /*#[derive(Accounts)]
    pub struct SendSol<'info> {
        #[account(mut)]
        pub user : Signer<'info>,
        #[account(mut)]
        pub base_account : Account<'info,BaseAccount>,
        //system_program : Program<'info, System>,
    }*/

    #[account]
    pub struct BaseAccount {
        pub total_gifs : u64,
        pub gif_list : Vec<ItemStruct>, 
    }

}





