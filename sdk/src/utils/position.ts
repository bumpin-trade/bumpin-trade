import {PositionBalance, PositionStatus, TradeToken, UserAccount, UserPosition,} from "../typedef";
import {OracleClient} from "../oracles/types";
import {BN} from "@coral-xyz/anchor";
import {BumpinTokenUtils} from "./token";
// @ts-ignore
import {isEqual} from "lodash";

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
        tradeTokens: TradeToken[]
    ): Promise<PositionBalance> {
        let totalBalance = {
            initialMarginUsd: new BN(0),
            positionUnPnl: new BN(0),
            mmUsd: new BN(0),
            initialMarginUsdFromPortfolio: new BN(0),
            positionFee: new BN(0) // TODO: Dean, add positionFee here
        };

        for (let userPosition of user.positions) {
            if (isEqual(userPosition.status, PositionStatus.INIT)) {
                continue;
            }
            let indexTradeToken = BumpinTokenUtils.getTradeTokenByMintPublicKey(
                userPosition.indexMintKey,
                tradeTokens
            );
            let unPnlValue = await BumpinPositionUtils.getPositionUnPnlValue(
                oracle,
                indexTradeToken,
                userPosition
            );
            totalBalance.positionUnPnl = totalBalance.positionUnPnl.add(unPnlValue);
            totalBalance.mmUsd = totalBalance.mmUsd.add(userPosition.mmUsd);
            totalBalance.initialMarginUsd = totalBalance.initialMarginUsd.add(
                userPosition.initialMarginUsd
            );
            totalBalance.initialMarginUsdFromPortfolio =
                totalBalance.initialMarginUsdFromPortfolio.add(
                    userPosition.initialMarginUsdFromPortfolio
                );
        }

        //TODO: Dean, cal positionFee here

        return totalBalance;
    }

    public static async getPositionUnPnlValue(
        oracle: OracleClient,
        indexTradeToken: TradeToken,
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
        }
        return unPnl;
    }
}
