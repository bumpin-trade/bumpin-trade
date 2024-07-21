import {
  UserAccount,
  UserOrderAccount,
  UserPositionAccount,
  UserStakeAccount,
  UserTokenAccount,
} from "../typedef";
// @ts-ignore
import { isEqual } from "lodash";
import {
  OrderStatus,
  PositionStatus,
  UserStakeStatus,
  UserTokenStatus,
} from "../beans/beans";

export class BumpinUserUtils {
  public static getMyStake(me: UserAccount): UserStakeAccount[] {
    return me.stakes.filter((stake) => {
      return isEqual(stake.userStakeStatus, UserStakeStatus.USING);
    });
  }

  public static getMyToken(me: UserAccount): UserTokenAccount[] {
    return me.tokens.filter((token) => {
      return isEqual(token.userTokenStatus, UserTokenStatus.USING);
    });
  }

  public static getMyPosition(
    me: UserAccount,
    isPortfolioMargin?: boolean
  ): UserPositionAccount[] {
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
  ): UserOrderAccount[] {
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
