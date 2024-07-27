import {PositionBalance, PositionFee} from '../typedef';
import {BN} from '@coral-xyz/anchor';
import {BumpinTokenUtils} from './token';
// @ts-ignore
import {isEqual} from 'lodash';
import {BumpinMarketNotFound, BumpinPoolNotFound} from '../errors';
import {BumpinMarketUtils} from './market';
import {BumpinPoolUtils} from './pool';
import {
    Market,
    Pool,
    PositionStatus,
    TradeToken,
    User,
    UserPosition,
} from '../beans/beans';
import BigNumber from 'bignumber.js';
import {TradeTokenComponent} from '../componets/tradeToken';

export class BumpinPositionUtils {
    // public static async reducePositionPortfolioBalance(
    //   position: UserPosition,
    //   amount: BN
    // ): Promise<BN> {
    //   let reduceInitialMarginUsd = amount
    //     .mul(position.initialMarginUsdFromPortfolio)
    //     .div(position.initialMargin);
    //   if (position.initialMarginUsdFromPortfolio.lte(reduceInitialMarginUsd)) {
    //     position.initialMarginUsdFromPortfolio = new BN(0);
    //     return position.initialMarginUsdFromPortfolio
    //       .mul(position.initialMargin)
    //       .div(position.initialMargin);
    //   } else {
    //     position.initialMarginUsdFromPortfolio =
    //       position.initialMarginUsdFromPortfolio.sub(reduceInitialMarginUsd);
    //     return amount;
    //   }
    // }

    public static async getUserPositionValue(
        tradeTokenComponent: TradeTokenComponent,
        user: User,
        tradeTokens: TradeToken[],
        markets: Market[],
        pools: Pool[],
        positionValue: boolean = true
    ): Promise<PositionBalance> {
        let totalBalance = {
            initialMarginUsd: BigNumber(0),
            positionUnPnl: BigNumber(0),
            mmUsd: BigNumber(0),
            initialMarginUsdFromPortfolio: BigNumber(0),
            positionFee: BigNumber(0),
        };

        for (let userPosition of user.positions) {
            if (isEqual(userPosition.status, PositionStatus.INIT)) {
                continue;
            }
            const indexTradeToken =
                BumpinTokenUtils.getTradeTokenByOraclePublicKey(
                    userPosition.indexMintOracle,
                    tradeTokens,
                );
            const marginTradeToken =
                BumpinTokenUtils.getTradeTokenByMintPublicKey(
                    userPosition.marginMintKey,
                    tradeTokens,
                );
            const unPnlValue = await BumpinPositionUtils.getPositionUnPnlValue(
                tradeTokenComponent,
                indexTradeToken,
                marginTradeToken,
                userPosition, positionValue
            );
            const market = BumpinMarketUtils.getMarketBySymbol(
                userPosition.symbol,
                markets,
            );
            if (!market) {
                throw new BumpinMarketNotFound(userPosition.symbol);
            }
            const pool = BumpinPoolUtils.getPoolByPublicKey(
                userPosition.isLong ? market.poolKey : market.stablePoolKey,
                pools,
            );
            if (!pool) {
                throw new BumpinPoolNotFound(
                    userPosition.isLong ? market.poolKey : market.stablePoolKey,
                );
            }
            const posFee = await BumpinPositionUtils.getPositionFee(
                tradeTokenComponent,
                userPosition,
                market,
                pool,
                marginTradeToken,
            );

            totalBalance.positionUnPnl =
                totalBalance.positionUnPnl.plus(unPnlValue);
            totalBalance.positionFee = totalBalance.positionFee.plus(
                posFee.totalUsd,
            );

            totalBalance.mmUsd = totalBalance.mmUsd.plus(userPosition.mmUsd);
            totalBalance.initialMarginUsd = totalBalance.initialMarginUsd.plus(
                userPosition.initialMarginUsd,
            );
            totalBalance.initialMarginUsdFromPortfolio =
                totalBalance.initialMarginUsdFromPortfolio.plus(
                    userPosition.initialMarginUsdFromPortfolio,
                );
        }

        return totalBalance;
    }

    //TODO, Dean: check this
    public static async getPositionUnPnlValue(
        tradeTokenComponent: TradeTokenComponent,
        indexTradeToken: TradeToken,
        marginTradeToken: TradeToken,
        position: UserPosition,
        positionValue: boolean = true
    ): Promise<BigNumber> {
        const price = tradeTokenComponent.getTradeTokenPricesByOracleKey(
            indexTradeToken.oracleKey,
            1,
        )[0].price!;
        let unPnl = BigNumber(0);
        if (!position.positionSize.isZero()) {
            if (position.isLong) {
                unPnl = position.positionSize
                    .multipliedBy(price - position.entryPrice.toNumber())
                    .div(position.entryPrice);
            } else {
                unPnl = position.positionSize
                    .multipliedBy(position.entryPrice.toNumber() - price)
                    .div(position.entryPrice);
            }
            if (positionValue) {
                if (unPnl.gt(BigNumber(0))) {
                    unPnl = unPnl.multipliedBy(marginTradeToken.discount);
                } else {
                    unPnl = unPnl.multipliedBy(
                        marginTradeToken.liquidationFactor + 1,
                    );
                }
            }
        }
        return unPnl;
    }

    //TODO: Dean: check this
    public static async getPositionFee(
        tradeTokenComponent: TradeTokenComponent,
        position: UserPosition,
        market: Market,
        pool: Pool,
        marginTradeToken: TradeToken,
    ): Promise<PositionFee> {
        let positionFee = {
            fundingFee: BigNumber(0),
            fundingFeeUsd: BigNumber(0),
            borrowingFee: BigNumber(0),
            borrowingFeeUsd: BigNumber(0),
            closeFeeUsd: BigNumber(0),
            totalUsd: BigNumber(0),
        };

        const price = (
            await tradeTokenComponent.getTradeTokenPricesByMintKey(
                position.marginMintKey,
            )
        ).price!;

        if (position.isLong) {
            positionFee.fundingFee =
                market.fundingFee.longFundingFeeAmountPerSize
                    .minus(position.openFundingFeeAmountPerSize)
                    .multipliedBy(position.positionSize);
            positionFee.fundingFeeUsd =
                positionFee.fundingFee.multipliedBy(price);
        } else {
            positionFee.fundingFeeUsd =
                market.fundingFee.shortFundingFeeAmountPerSize
                    .minus(position.openFundingFeeAmountPerSize)
                    .multipliedBy(position.positionSize);
            positionFee.fundingFee =
                positionFee.fundingFeeUsd.multipliedBy(price);
        }

        positionFee.borrowingFee =
            pool.borrowingFee.cumulativeBorrowingFeePerToken
                .minus(position.openBorrowingFeePerToken)
                .multipliedBy(position.holdPoolAmount);
        positionFee.borrowingFeeUsd =
            positionFee.borrowingFee.multipliedBy(price);
        positionFee.closeFeeUsd = position.closeFeeInUsd;
        positionFee.totalUsd = positionFee.fundingFeeUsd
            .plus(positionFee.borrowingFeeUsd)
            .plus(positionFee.closeFeeUsd);
        return positionFee;
    }
}
