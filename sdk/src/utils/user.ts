import {
  OrderStatusAccount,
  PositionStatusAccount,
  UserAccount,
  UserOrderAccount,
  UserPositionAccount,
  UserStakeAccount,
  UserStakeStatusAccount,
  UserTokenAccount,
  UserTokenStatusAccount,
} from "../typedef";
// @ts-ignore
import { isEqual } from "lodash";

export class BumpinUserUtils {
  public static getMyStake(me: UserAccount): UserStakeAccount[] {
    return me.stakes.filter((stake) => {
      return isEqual(stake.userStakeStatus, UserStakeStatusAccount.USING);
    });
  }

  public static getMyToken(me: UserAccount): UserTokenAccount[] {
    return me.tokens.filter((token) => {
      return isEqual(token.userTokenStatus, UserTokenStatusAccount.USING);
    });
  }

  public static getMyPosition(
    me: UserAccount,
    isPortfolioMargin?: boolean
  ): UserPositionAccount[] {
    return me.positions.filter((position) => {
      if (!isEqual(position.status, PositionStatusAccount.USING)) {
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
      if (!isEqual(order.status, OrderStatusAccount.USING)) {
        return false;
      }
      if (isPortfolioMargin) {
        return order.isPortfolioMargin === isPortfolioMargin;
      }
      return true;
    });
  }
}
