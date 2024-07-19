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

  public static getMyPosition(me: UserAccount): UserPosition[] {
    return me.positions.filter((position) => {
      return isEqual(position.status, PositionStatus.USING);
    });
  }

  public static getMyOrder(me: UserAccount): UserOrder[] {
    return me.orders.filter((order) => {
      return isEqual(order.status, OrderStatus.USING);
    });
  }
}
