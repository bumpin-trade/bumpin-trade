use anchor_lang::prelude::*;
pub mod pc;
mod pcv2;

use crate::pcv2::PriceUpdateV2;
use pc::Price;

declare_id!("ECKhW7wvKQGGhzGFS7LqGv4z3DRoAD8HJywd25XjBoxP");

#[program]
pub mod pyth {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        price: i64,
        exponent: i32,
        conf: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let oracle = &ctx.accounts.price;
        let mut price_oracle = Price::load(oracle).unwrap();

        price_oracle.magic = 2712847316;
        price_oracle.ver = 2;
        price_oracle.atype = 3;
        price_oracle.size = 3312;
        price_oracle.ptype = pc::PriceType::Price;
        price_oracle.exponent = exponent;
        price_oracle.valid_slot = clock.slot;
        price_oracle.timestamp = clock.unix_timestamp;

        price_oracle.agg.price = price;
        price_oracle.agg.conf = conf;
        price_oracle.agg.status = pc::PriceStatus::Trading;
        price_oracle.agg.corp_act = pc::CorpAction::NoCorpAct;
        price_oracle.agg.pub_slot = clock.slot;

        price_oracle.ema_price.val = price;
        price_oracle.ema_price.numer = price;
        price_oracle.ema_price.denom = 1;

        price_oracle.ema_conf.val = conf as i64;
        price_oracle.ema_conf.numer = conf as i64;
        price_oracle.ema_conf.denom = 1;
        Ok(())
    }

    pub fn set_price(ctx: Context<SetPrice>, price: i64, conf: u64) -> Result<()> {
        let clock = Clock::get()?;
        let oracle = &ctx.accounts.price;
        let mut price_oracle = Price::load(oracle).unwrap();

        price_oracle.ema_price.val = price;
        price_oracle.ema_price.numer = price;
        price_oracle.ema_price.denom = 1;

        price_oracle.ema_conf.val = conf as i64;
        price_oracle.ema_conf.numer = conf as i64;
        price_oracle.ema_conf.denom = 1;

        price_oracle.agg.price = price;
        price_oracle.agg.conf = conf;

        price_oracle.agg.pub_slot = clock.slot;
        price_oracle.timestamp = clock.unix_timestamp;

        Ok(())
    }

    pub fn initialize_v2(ctx: Context<InitializeV2>, params: InitializeV2Params) -> Result<()> {
        let price_update_v2 = &mut ctx.accounts.price_update_v2;
        price_update_v2.price_message.feed_id = params.feed_id.to_bytes();
        price_update_v2.price_message.price = params.price;
        price_update_v2.price_message.exponent = params.exponent;
        price_update_v2.verification_level = pcv2::VerificationLevel::Full;
        msg!("initialize_v2, id: {}, params.feed_id: {}", params.id, params.feed_id);
        msg!("PriceUpdateV2 initialized");
        Ok(())
    }

    pub fn set_price_v2(ctx: Context<SetPriceV2>, price: i64, conf: u64) -> Result<()> {
        let price_update_v2 = &mut ctx.accounts.price_update_v2;
        price_update_v2.price_message.price = price;
        price_update_v2.price_message.prev_publish_time =
            price_update_v2.price_message.publish_time;
        price_update_v2.price_message.publish_time = Clock::get()?.unix_timestamp;
        price_update_v2.price_message.conf = conf;
        Ok(())
    }
}
#[derive(Accounts)]
pub struct SetPrice<'info> {
    /// CHECK: this program is just for testing
    #[account(mut)]
    pub price: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /// CHECK: this program is just for testing
    #[account(mut)]
    pub price: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct SetPriceV2<'info> {
    #[account(mut)]
    pub price_update_v2: Account<'info, PriceUpdateV2>,
}

#[derive(Accounts)]
#[instruction(params: InitializeV2Params)]
pub struct InitializeV2<'info> {
    #[account(
        init,
        seeds = [b"pythv2", params.id.to_le_bytes().as_ref()],
        space =  std::mem::size_of::<PriceUpdateV2>() ,
        bump,
        payer = admin
    )]
    pub price_update_v2: Account<'info, PriceUpdateV2>,

    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Copy)]
pub struct InitializeV2Params {
    pub feed_id: Pubkey,
    pub price: i64,
    pub exponent: i32,
    pub id: u16,
}
