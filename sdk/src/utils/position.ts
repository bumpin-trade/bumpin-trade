import {PositionBalance, PositionStatus, TradeToken, UserAccount, UserPosition} from "../types";
import {OracleClient} from "../oracles/types";
import {BN} from "@coral-xyz/anchor";
import {BumpinTokenUtils} from "./token";

export class BumpinPositionUtils {

    public static async reducePositionPortfolioBalance(position: UserPosition, amount: BN): Promise<BN> {
        let reduceInitialMarginUsd = amount.mul(position.initialMarginUsdFromPortfolio).div(position.initialMargin);
        if (position.initialMarginUsdFromPortfolio.lte(reduceInitialMarginUsd)) {
            position.initialMarginUsdFromPortfolio = new BN(0);
            return position.initialMarginUsdFromPortfolio.mul(position.initialMargin).div(position.initialMargin);
        } else {
            position.initialMarginUsdFromPortfolio = position.initialMarginUsdFromPortfolio.sub(reduceInitialMarginUsd);
            return amount;
        }
    }

    public static async getUserPositionValue(oracle: OracleClient, user:UserAccount, tradeTokens: TradeToken[]): Promise<PositionBalance> {
        let totalBalance = {
            initialMarginUsdFromPortfolio: new BN(0),
            positionUnPnl: new BN(0),
            mmUsd: new BN(0)
        };

        for (let userPosition of user.userPositions) {
            if (userPosition.status === PositionStatus.INIT) {
                continue;
            }
            let indexTradeToken =  BumpinTokenUtils.getTradeTokenByMintPublicKey(userPosition.indexMint,tradeTokens);
            let unPnlValue = await BumpinPositionUtils.getPositionUnPnlValue(oracle, indexTradeToken, userPosition);
            totalBalance.positionUnPnl = totalBalance.positionUnPnl.add(unPnlValue);
            totalBalance.mmUsd = totalBalance.mmUsd.add(userPosition.mmUsd);
            totalBalance.initialMarginUsdFromPortfolio = totalBalance.initialMarginUsdFromPortfolio.add(userPosition.initialMarginUsdFromPortfolio);
        }

        return totalBalance;
    }


    public static async getPositionUnPnlValue(oracle: OracleClient, indexTradeToken: TradeToken, position: UserPosition): Promise<BN> {
        let priceData = await oracle.getOraclePriceData(indexTradeToken.oracle);
        let unPnl = new BN(0);
        if (!position.positionSize.isZero()) {
            if (position.isLong) {
                unPnl = position.positionSize.mul(priceData.price.sub(position.entryPrice)).div(position.entryPrice);
            } else {
                unPnl = position.positionSize.mul(position.entryPrice.sub(priceData.price)).div(position.entryPrice);
            }
        }
        return unPnl;

    }
}