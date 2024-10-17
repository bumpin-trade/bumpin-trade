use anchor_lang::prelude::*;
pub mod pc;
mod pcv2;

use crate::pcv2::{PriceUpdateV2};
use pc::Price;

#[cfg(feature = "local-net")]
declare_id!("6GHM4TvoUsUxJp5mHC44TBmx8J5gTSQUHEtJgBvHWWXP");
#[cfg(not(feature = "local-net"))]
declare_id!("CC1ePebfvPy7QRTimPoVecS2UsBvYv46ynrzWocc92s");

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
        let oracle = &ctx.accounts.price_update_v2;
        let mut price_update_v2 =PriceUpdateV2::load(oracle).unwrap();
        price_update_v2.price_message.feed_id = params.feed_id.to_bytes();
        price_update_v2.price_message.price = params.price;
        price_update_v2.price_message.exponent = params.exponent;
        msg!("initialize_v2, params.feed_id:{}", params.feed_id);
        msg!("initialize_v2, params.feed_id.to_bytes(:{:?}", params.feed_id.to_bytes());
        msg!("initialize_v2, price_update_v2.price_message.feed_id:{:?}", price_update_v2.price_message.feed_id);
        msg!("PriceUpdateV2 initialized");
        Ok(())
    }

    pub fn set_price_v2(ctx: Context<SetPriceV2>, price: i64, conf: u64) -> Result<()> {
        let oracle = &ctx.accounts.price_update_v2;
        let mut price_update_v2 = PriceUpdateV2::load(oracle).unwrap();
        price_update_v2.price_message.price = price;
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
    /// CHECK: this program is just for testing
    #[account(mut)]
    pub price_update_v2: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct InitializeV2<'info> {
    /// CHECK: this program is just for testing
    #[account(mut)]
    pub price_update_v2: AccountInfo<'info>,
}

#[derive(AnchorDeserialize, AnchorSerialize, Debug, Clone, Copy)]
pub struct InitializeV2Params {
    pub feed_id: Pubkey,
    pub price: i64,
    pub exponent: i32,
}
