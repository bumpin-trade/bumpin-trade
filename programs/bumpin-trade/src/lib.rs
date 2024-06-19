use std::iter::Peekable;
use std::slice::Iter;

use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::{Token, TokenAccount};

use instructions::*;

use crate::instructions::admin::initialize_state::handle_initialize_state;
use crate::processor::optional_accounts::{load_maps, AccountMaps};
use crate::state::infrastructure::user_order::UserOrder;
use crate::state::vault_map::VaultMap;
use crate::traits::Size;

pub mod errors;
pub mod ids;
pub mod instructions;
pub mod macros;
pub mod math;
pub mod processor;
pub mod state;
pub mod traits;
pub mod utils;

declare_id!("GhzHdLjZ1qLLPnPq6YdeqJAszuBRN8WnLnK455yBbig6");

#[program]
pub mod bumpin_trade {
    use super::*;

    pub fn initialize1<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Initialize1>,
    ) -> Result<()> {
        msg!("initialize1");

        // let key_value = &ctx.accounts.key_value;

        // 打印 KeyValue 的已知字段
        // msg!("key_value key: {:?}", key_value.key);
        // msg!("key_value value: {:?}", key_value.value);
        // for x in ctx.remaining_accounts.iter() {
        //     let a: Account<TokenAccount> = Account::try_from(x).unwrap();
        //     msg!("a: {:?}", a);
        // }

        let r = VaultMap::load(&mut ctx.remaining_accounts.iter().peekable());
        msg!("r: {}", r.is_ok());
        let u = r.unwrap();
        msg!("len: {}", u.0.len());
        u.0.iter().for_each(|(k, v)| {
            msg!("k: {:?}", k);
            msg!("v: {:?}", v);
        });
        Ok(())
    }

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("initialize");

        let trade_token_vault = &ctx.accounts.trade_token_vault;

        // 打印 TokenAccount 的已知字段
        msg!("trade_token_vault owner: {:?}", trade_token_vault.owner);
        msg!("trade_token_vault mint: {:?}", trade_token_vault.mint);
        msg!("trade_token_vault amount: {:?}", trade_token_vault.amount);

        let binding = trade_token_vault.to_account_info();
        // let account :Account<TokenAccount> = Account::try_from(&binding).unwrap();

        // let data1 = binding.try_borrow_data().unwrap();
        // let d1 =  array_ref![data1, 0, 8];
        msg!("data1: {:?}", binding);
        msg!("data1 owner: {:?}", binding.owner);
        Ok(())
    }

    pub fn initialize_state<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeState>,
        param: InitializeStateParams,
    ) -> Result<()> {
        handle_initialize_state(ctx, param)
    }

    pub fn initialize_pool<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializePool>,
        name: [u8; 32],
    ) -> Result<()> {
        handle_initialize_pool(ctx, name)
    }

    pub fn initialize_user<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeUser>,
    ) -> Result<()> {
        handle_initialize_user(ctx)
    }

    pub fn initialize_trade_token<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeTradeToken>,
        discount: u128,
        liquidation_factor: u128,
    ) -> Result<()> {
        handle_initialize_trade_token(ctx, discount, liquidation_factor)
    }

    pub fn initialize_market<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, InitializeMarket>,
        symbol: [u8; 32],
    ) -> Result<()> {
        handle_initialize_market(ctx, symbol)
    }

    /*-----pool pool------*/
    pub fn pool_stake<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PoolStake>,
        params: StakeParams,
    ) -> Result<()> {
        handle_pool_stake(ctx, params)
    }

    pub fn pool_un_stake<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PoolUnStake>,
        params: UnStakeParams,
    ) -> Result<()> {
        handle_pool_un_stake(ctx, params)
    }

    /*-----account------*/
    pub fn deposit<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Deposit>,
        token_index: u16,
        amount: u128,
    ) -> Result<()> {
        handle_deposit(ctx, token_index, amount)
    }

    pub fn withdraw<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, Withdraw>,
        amount: u128,
    ) -> Result<()> {
        handle_withdraw(ctx, amount)
    }

    /*-----order------*/
    pub fn place_order<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PlaceOrder>,
        params: PlaceOrderParams,
    ) -> Result<()> {
        handle_place_order(ctx, params)
    }

    pub fn execute_order<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, PlaceOrder>,
        order_id: u128,
    ) -> Result<()> {
        let user_account_loader = &ctx.accounts.user_account;
        let margin_token_account = &ctx.accounts.margin_token;
        let pool_account_loader = &ctx.accounts.pool;
        let stable_pool_account_loader = &ctx.accounts.stable_pool;
        let market_account_loader = &ctx.accounts.market;
        let state_account = &ctx.accounts.state;
        let user_token_account = &ctx.accounts.user_token_account;
        let pool_vault_account = &ctx.accounts.pool_vault;
        let stable_pool_vault_account = &ctx.accounts.stable_pool_vault;
        let trade_token_loader = &ctx.accounts.trade_token;
        let trade_token_vault_account = &ctx.accounts.trade_token_vault;
        let bump_signer_account_info = &ctx.accounts.bump_signer;
        let token_program = &ctx.accounts.token_program;
        let remaining_accounts_iter: &mut Peekable<Iter<'info, AccountInfo<'info>>> =
            &mut ctx.remaining_accounts.iter().peekable();
        let AccountMaps { trade_token_map, mut oracle_map, .. } =
            load_maps(remaining_accounts_iter)?;

        handle_execute_order(
            user_account_loader,
            margin_token_account,
            pool_account_loader,
            stable_pool_account_loader,
            market_account_loader,
            state_account,
            user_token_account,
            pool_vault_account,
            stable_pool_vault_account,
            trade_token_loader,
            trade_token_vault_account,
            bump_signer_account_info,
            token_program,
            ctx.program_id,
            &trade_token_map,
            &mut oracle_map,
            &mut UserOrder::default(),
            order_id,
            false,
        )
    }

    pub fn cancel_order(ctx: Context<CancelOrderCtx>, order_id: u128) -> Result<()> {
        handle_cancel_order(ctx, order_id)
    }

    /*-----position------*/
    pub fn add_position_margin<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, AddPositionMargin>,
        params: UpdatePositionMarginParams,
    ) -> Result<()> {
        handle_add_position_margin(ctx, params)
    }

    pub fn update_position_leverage<'a, 'b, 'c: 'info, 'info>(
        ctx: Context<'a, 'b, 'c, 'info, UpdatePositionLeverage>,
        params: UpdatePositionLeverageParams,
    ) -> Result<()> {
        handle_update_position_leverage(ctx, params)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        seeds = [b"trade_token_vault".as_ref()],
        bump,
        payer = admin,
        token::mint = trade_token_mint,
        token::authority = bump_signer
    )]
    pub trade_token_vault: Box<Account<'info, TokenAccount>>,
    /// CHECK: ?
    #[account()]
    pub bump_signer: AccountInfo<'info>,
    pub trade_token_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
}

#[account]
pub struct KeyValue {
    pub key: String,
    pub value: String,
}

impl Size for KeyValue {
    const SIZE: usize = std::mem::size_of::<KeyValue>() + 8;
}

#[derive(Accounts)]
pub struct Initialize1<'info> {
    #[account(init, payer = user, space = KeyValue::SIZE)]
    pub key_value: Account<'info, KeyValue>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
