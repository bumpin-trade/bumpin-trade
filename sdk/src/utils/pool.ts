import { PublicKey } from '@solana/web3.js';
import { BumpinPoolNotFound } from '../errors';
import { Pool, State } from '../beans/beans';
import BigNumber from 'bignumber.js';

export class BumpinPoolUtils {
    private static getGapInSeconds = (lastUpdate: number): number => {
        return Math.floor(Date.now() / 1000) - lastUpdate;
    };
    public static getPoolByMintPublicKey(mint: PublicKey, pools: Pool[]): Pool {
        let pool = pools.find((pool) => {
            return pool.mintKey.equals(mint);
        });
        if (pool === undefined) {
            throw new BumpinPoolNotFound(mint);
        }
        return pool;
    }

    public static getPoolByIndex(index: number, pools: Pool[]): Pool {
        let pool = pools.find((pool) => {
            return pool.index === index;
        });
        if (pool === undefined) {
            throw new BumpinPoolNotFound(new PublicKey(''));
        }
        return pool;
    }

    public static getPoolByPublicKey(poolKey: PublicKey, pools: Pool[]): Pool {
        let pool = pools.find((pool) => {
            return pool.key.equals(poolKey);
        });
        if (pool === undefined) {
            throw new BumpinPoolNotFound(new PublicKey(''));
        }
        return pool;
    }

    public static getPoolBorrowingFeeDelta(pool: Pool) {
        if (
            pool.balance.amount.isZero() &&
            pool.balance.unSettleAmount.isZero()
        ) {
            return new BigNumber(0);
        }
        let gapInSeconds = BumpinPoolUtils.getGapInSeconds(
            pool.borrowingFee.updatedAt.toNumber(),
        );
        let total_amount = pool.balance.amount.plus(
            pool.balance.unSettleAmount,
        );
        let hold_rate = pool.balance.holdAmount.dividedBy(total_amount);
        let borrowing_fee_rate_per_second = hold_rate.multipliedBy(
            pool.config.borrowingInterestRate,
        );
        return borrowing_fee_rate_per_second.multipliedBy(gapInSeconds);
    }
}
