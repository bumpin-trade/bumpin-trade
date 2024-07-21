import {
    Market,
    Pool,
    PositionBalance,
    PositionFee,
    PositionStatus,
    TradeToken,
    UserAccount,
    UserPosition,
} from "../typedef";
import {OracleClient} from "../oracles/types";
import {BN} from "@coral-xyz/anchor";
import {BumpinTokenUtils} from "./token";
// @ts-ignore
import {isEqual} from "lodash";
import {PublicKey} from "@solana/web3.js";
import {BumpinMarketNotFound, BumpinPoolNotFound} from "../errors";

export class BumpinPositionUtils {
    public static async reducePositionPortfolioBalance(
        position: UserPosition,
        amount: BN
    ): Promise<BN> {
        let reduceInitialMarginUsd = amount
            .mul(position.initialMarginUsdFromPortfolio)
            .div(position.initialMargin);
        if (position.initialMarginUsdFromPortfolio.lte(reduceInitialMarginUsd)) {
            position.initialMarginUsdFromPortfolio = new BN(0);
            return position.initialMarginUsdFromPortfolio
                .mul(position.initialMargin)
                .div(position.initialMargin);
        } else {
            position.initialMarginUsdFromPortfolio =
                position.initialMarginUsdFromPortfolio.sub(reduceInitialMarginUsd);
            return amount;
        }
    }

    public static async getUserPositionValue(
        oracle: OracleClient,
        user: UserAccount,
        tradeTokens: TradeToken[],
        marketMap: Map<number[], Market>,
        poolMap: Map<PublicKey, Pool>
    ): Promise<PositionBalance> {
        let totalBalance = {
            initialMarginUsd: new BN(0),
            positionUnPnl: new BN(0),
            mmUsd: new BN(0),
            initialMarginUsdFromPortfolio: new BN(0),
            positionFee: new BN(0),
        };

        for (let userPosition of user.positions) {
            if (isEqual(userPosition.status, PositionStatus.INIT)) {
                continue;
            }
            let indexTradeToken = BumpinTokenUtils.getTradeTokenByOraclePublicKey(
                userPosition.indexMintOracle,
                tradeTokens
            );
            let marginTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
                userPosition.marginMintKey,
                tradeTokens
            );
            let unPnlValue = await BumpinPositionUtils.getPositionUnPnlValue(
                oracle,
                indexTradeToken,
                marginTradeToken,
                userPosition
            );
            let market = marketMap.get(userPosition.symbol);
            if (!market) {
                throw new BumpinMarketNotFound(userPosition.symbol);
            }
            let pool = poolMap.get(userPosition.isLong ? market.poolKey : market.stablePoolKey);
            if (!pool) {
                throw new BumpinPoolNotFound(userPosition.isLong ? market.poolKey : market.stablePoolKey);
            }
            let posFee = await BumpinPositionUtils.getPositionFee(oracle,
                userPosition, market, pool, marginTradeToken
            );

            totalBalance.positionUnPnl = totalBalance.positionUnPnl.add(unPnlValue);
            totalBalance.positionFee = totalBalance.positionFee.add(posFee.totalUsd);

            totalBalance.mmUsd = totalBalance.mmUsd.add(userPosition.mmUsd);
            totalBalance.initialMarginUsd = totalBalance.initialMarginUsd.add(
                userPosition.initialMarginUsd
            );
            totalBalance.initialMarginUsdFromPortfolio =
                totalBalance.initialMarginUsdFromPortfolio.add(
                    userPosition.initialMarginUsdFromPortfolio
                );
        }

        return totalBalance;
    }

    public static async getPositionUnPnlValue(
        oracle: OracleClient,
        indexTradeToken: TradeToken,
        marginTradeToken: TradeToken,
        position: UserPosition
    ): Promise<BN> {
        let priceData = await oracle.getOraclePriceData(indexTradeToken.oracleKey);
        let unPnl = new BN(0);
        if (!position.positionSize.isZero()) {
            if (position.isLong) {
                unPnl = position.positionSize
                    .mul(priceData.price.sub(position.entryPrice))
                    .div(position.entryPrice);
            } else {
                unPnl = position.positionSize
                    .mul(position.entryPrice.sub(priceData.price))
                    .div(position.entryPrice);
            }
            if (unPnl.gt(new BN(0))) {
                unPnl = unPnl.mulRate(new BN(marginTradeToken.discount));
            } else {
                unPnl = unPnl.mulRate(new BN(marginTradeToken.liquidationFactor).add(new BN(1)));
            }
        }
        return unPnl;
    }

    public static async getPositionFee(oracle: OracleClient,
                                       position: UserPosition, market: Market, pool: Pool, marginTradeToken: TradeToken,
    ): Promise<PositionFee> {
        let positionFee = {
            fundingFee: new BN(0),
            fundingFeeUsd: new BN(0),
            borrowingFee: new BN(0),
            borrowingFeeUsd: new BN(0),
            closeFeeUsd: new BN(0),
            totalUsd: new BN(0),
        }
        let priceData = await oracle.getOraclePriceData(position.marginMintKey);

        if (position.isLong) {
            positionFee.fundingFee = market.fundingFee.longFundingFeeAmountPerSize.sub(position.openFundingFeeAmountPerSize).mulSmallRate(position.positionSize);
            positionFee.fundingFeeUsd = positionFee.fundingFee.toUsd(priceData.price, marginTradeToken.decimals);
        } else {
            positionFee.fundingFeeUsd = market.fundingFee.shortFundingFeeAmountPerSize.sub(position.openFundingFeeAmountPerSize).mulSmallRate(position.positionSize);
            positionFee.fundingFee = positionFee.fundingFeeUsd.toToken(priceData.price, marginTradeToken.decimals);
        }

        positionFee.borrowingFee = pool.borrowingFee.cumulativeBorrowingFeePerToken.sub(position.openBorrowingFeePerToken).mulSmallRate(position.holdPoolAmount);
        positionFee.borrowingFeeUsd = positionFee.borrowingFee.toUsd(priceData.price, marginTradeToken.decimals);
        positionFee.closeFeeUsd = position.closeFeeInUsd;
        positionFee.totalUsd = positionFee.fundingFeeUsd.add(positionFee.borrowingFeeUsd).add(positionFee.closeFeeUsd);
        return positionFee;
    }
}
