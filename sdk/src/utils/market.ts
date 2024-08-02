import { BumpinAccountNotFound } from '../errors';
import { PublicKey } from '@solana/web3.js';
import { Market, State } from '../beans/beans';
import BigNumber from 'bignumber.js';

export class BumpinMarketUtils {
    private static getGapInSeconds = (lastUpdate: number): number => {
        return Math.floor(Date.now() / 1000) - lastUpdate;
    };

    public static getMarketByIndex(index: number, markets: Market[]): Market {
        let market = markets.find((market) => {
            return market.index === index;
        });
        if (market === undefined) {
            throw new BumpinAccountNotFound('Market: ' + index);
        }
        return market;
    }

    public static getMarketsByPoolKey(
        poolKey: PublicKey,
        markets: Market[],
    ): Market[] {
        return markets.filter((market) => {
            return market.poolKey.equals(poolKey);
        });
    }

    public static getMarketBySymbol(symbol: string, markets: Market[]): Market {
        let market = markets.find((market) => {
            return market.symbol === symbol;
        });
        if (market === undefined) {
            throw new BumpinAccountNotFound('Market: ' + symbol);
        }
        return market;
    }

    public static getMarketPerTokenDelta(
        market: Market,
        state: State,
        baseTokenPrice: number,
    ) {
        let funding_fee_duration_in_seconds = BumpinMarketUtils.getGapInSeconds(
            market.fundingFee.updatedAt.toNumber(),
        );
        let long_funding_fee_per_qty_delta = new BigNumber(0);
        let short_funding_fee_per_qty_delta = new BigNumber(0);
        let long = market.longOpenInterest;
        let short = market.shortOpenInterest;
        if (
            (long.openInterest.isZero() && short.openInterest.isZero()) ||
            long.openInterest.eq(short.openInterest) ||
            funding_fee_duration_in_seconds == 0
        ) {
            return {
                longDelta: long_funding_fee_per_qty_delta,
                shortDelta: short_funding_fee_per_qty_delta,
            };
        }

        let long_pay_short = long.openInterest.gt(short.openInterest);
        let funding_rate_per_second;
        let long_position_interest = long.openInterest;
        let short_position_interest = short.openInterest;
        let diff = long_position_interest.minus(short_position_interest).abs();
        let open_interest = long_position_interest.plus(
            short_position_interest,
        );
        if (diff.isZero() || open_interest.isZero()) {
            funding_rate_per_second = new BigNumber(0);
        } else {
            funding_rate_per_second = diff
                .multipliedBy(state.fundingFeeBaseRate)
                .dividedBy(open_interest);
        }
        let total_funding_fee = BigNumber.max(
            long.openInterest,
            short.openInterest,
        )
            .multipliedBy(funding_fee_duration_in_seconds)
            .multipliedBy(funding_rate_per_second);
        if (long.openInterest.gt(new BigNumber(0))) {
            let current_long_funding_fee_per_qty = long_pay_short
                ? total_funding_fee.dividedBy(long.openInterest)
                : BigNumber.min(
                      state.fundingFeeBaseRate.multipliedBy(
                          funding_fee_duration_in_seconds,
                      ),
                      total_funding_fee.dividedBy(long.openInterest),
                  );
            long_funding_fee_per_qty_delta = long_pay_short
                ? current_long_funding_fee_per_qty.dividedBy(baseTokenPrice)
                : current_long_funding_fee_per_qty
                      .dividedBy(baseTokenPrice)
                      .negated();
        }

        if (short.openInterest.gt(new BigNumber(0))) {
            short_funding_fee_per_qty_delta = long_pay_short
                ? BigNumber.min(
                      state.fundingFeeBaseRate.multipliedBy(
                          funding_fee_duration_in_seconds,
                      ),
                      total_funding_fee.div(short.openInterest),
                  ).negated()
                : total_funding_fee.div(short.openInterest);
        }
        return {
            longDelta: long_funding_fee_per_qty_delta,
            shortDelta: short_funding_fee_per_qty_delta,
        };
    }
}
