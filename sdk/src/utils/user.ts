// @ts-ignore
import { isEqual } from 'lodash';
import {
    OrderStatus,
    PositionStatus,
    User,
    UserOrder,
    UserPosition,
    UserStake,
    UserStakeStatus,
    UserToken,
    UserTokenStatus,
} from '../beans/beans';
import { PublicKey } from '@solana/web3.js';

export class BumpinUserUtils {
    public static getMyStake(me: User): UserStake[] {
        return me.stakes.filter((stake) => {
            return isEqual(stake.userStakeStatus, UserStakeStatus.USING);
        });
    }

    public static getMyTokenByMint(
        me: User,
        mint: PublicKey,
    ): UserToken | undefined {
        let userToken = me.tokens.find((token) => {
            return (
                isEqual(token.userTokenStatus, UserTokenStatus.USING) &&
                token.tokenMintKey.equals(mint)
            );
        });
        return userToken;
    }

    public static getMyToken(me: User): UserToken[] {
        return me.tokens.filter((token) => {
            return isEqual(token.userTokenStatus, UserTokenStatus.USING);
        });
    }

    public static getMyPosition(
        me: User,
        isPortfolioMargin?: boolean,
    ): UserPosition[] {
        return me.positions.filter((position) => {
            if (!isEqual(position.status, PositionStatus.USING)) {
                return false;
            }
            if (isPortfolioMargin != null) {
                return position.isPortfolioMargin === isPortfolioMargin;
            }
            return true;
        });
    }

    public static getMyOrder(
        me: User,
        isPortfolioMargin?: boolean,
    ): UserOrder[] {
        return me.orders.filter((order) => {
            if (!isEqual(order.status, OrderStatus.USING)) {
                return false;
            }
            if (isPortfolioMargin != null) {
                return order.isPortfolioMargin === isPortfolioMargin;
            }
            return true;
        });
    }
}
