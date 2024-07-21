import {
  OrderStatus,
  PositionStatus,
  UserAccount,
  UserOrder,
  UserPosition,
  UserStake,
  UserStakeStatus,
  UserToken,
  UserTokenStatus,
} from "../typedef";
// @ts-ignore
import { isEqual } from "lodash";

export class BumpinUserUtils {
  public static getMyStake(me: UserAccount): UserStake[] {
    return me.stakes.filter((stake) => {
      return isEqual(stake.userStakeStatus, UserStakeStatus.USING);
    });
  }

  public static getMyToken(me: UserAccount): UserToken[] {
    return me.tokens.filter((token) => {
      return isEqual(token.userTokenStatus, UserTokenStatus.USING);
    });
  }

  public static getMyPosition(
    me: UserAccount,
    isPortfolioMargin?: boolean
  ): UserPosition[] {
    return me.positions.filter((position) => {
      if (!isEqual(position.status, PositionStatus.USING)) {
        return false;
      }
      if (isPortfolioMargin) {
        return position.isPortfolioMargin === isPortfolioMargin;
      }
      return true;
    });
  }

  public static getMyOrder(
    me: UserAccount,
    isPortfolioMargin?: boolean
  ): UserOrder[] {
    return me.orders.filter((order) => {
      if (!isEqual(order.status, OrderStatus.USING)) {
        return false;
      }
      if (isPortfolioMargin) {
        return order.isPortfolioMargin === isPortfolioMargin;
      }
      return true;
    });
  }
}
