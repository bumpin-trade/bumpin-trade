import BN from "bn.js";

export class OrderSide {
    static readonly NONE = {none: {}};
    static readonly LONG = {long: {}};
    static readonly SHORT = {short: {}};

}

export type OrderSideValue =
    | typeof OrderSide.NONE
    | typeof OrderSide.LONG
    | typeof OrderSide.SHORT;


export class OrderStatus {
    static readonly INIT = {init: {}};
    static readonly USING = {using: {}};
}

export type OrderStatusValue =
    | typeof OrderStatus.INIT
    | typeof OrderStatus.USING;

export class PositionSide {
    static readonly NONE = {none: {}};
    static readonly INCREASE = {increase: {}};
    static readonly DECREASE = {decrease: {}};
}

export type PositionSideValue =
    | typeof PositionSide.NONE
    | typeof PositionSide.INCREASE
    | typeof PositionSide.DECREASE;


export class OrderType {
    static readonly NONE = {none: {}};
    static readonly MARKET = {market: {}};
    static readonly LIMIT = {limit: {}};
    static readonly STOP = {stop: {}};
}

export type OrderTypeValue =
    | typeof OrderType.NONE
    | typeof OrderType.MARKET
    | typeof OrderType.LIMIT
    | typeof OrderType.STOP;

export class StopType {
    static readonly NONE = {none: {}};
    static readonly StopLoss = {stopLoss: {}};
    static readonly TakeProfit = {takeProfit: {}};
}

export type StopTypeValue =
    | typeof StopType.NONE
    | typeof StopType.StopLoss
    | typeof StopType.TakeProfit;

/**
 * #[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Eq, PartialEq)]
 * pub struct PlaceOrderParams {
 *     pub symbol: [u8; 32],
 *     pub is_cross_margin: bool,
 *     pub is_native_token: bool,
 *     pub order_side: OrderSide,
 *     pub position_side: PositionSide,
 *     pub order_type: OrderType,
 *     pub stop_type: StopType,
 *     pub size: u128,
 *     pub order_margin: u128,
 *     pub leverage: u128,
 *     pub trigger_price: u128,
 *     pub acceptable_price: u128,
 *     pub place_time: u128,
 * }
 */

export type PlaceOrderParams = {
    symbol: number[];
    isCrossMargin: boolean;
    isNativeToken: boolean;
    orderSide: OrderSideValue;
    positionSide: PositionSideValue;
    orderType: OrderTypeValue;
    stopType: StopTypeValue;
    size: BN;
    orderMargin: BN;
    leverage: BN;
    triggerPrice: BN;
    acceptablePrice: BN;
    placeTime: BN;
    poolIndex: number;
    stablePoolIndex: number,
    marketIndex: number,
    tradeTokenIndex: number,
    indexTradeTokenIndex: number,


    // constructor(
    //     symbol: Uint8Array,
    //     isCrossMargin: boolean,
    //     isNativeToken: boolean,
    //     orderSide: OrderSide,
    //     positionSide: PositionSide,
    //     orderType: OrderType,
    //     stopType: StopType,
    //     size: BN,
    //     orderMargin: BN,
    //     leverage: BN,
    //     triggerPrice: BN,
    //     acceptablePrice: BN,
    //     placeTime: BN
    // ) {
    //     this.symbol = symbol;
    //     this.isCrossMargin = isCrossMargin;
    //     this.isNativeToken = isNativeToken;
    //     this.orderSide = orderSide;
    //     this.positionSide = positionSide;
    //     this.orderType = orderType;
    //     this.stopType = stopType;
    //     this.size = size;
    //     this.orderMargin = orderMargin;
    //     this.leverage = leverage;
    //     this.triggerPrice = triggerPrice;
    //     this.acceptablePrice = acceptablePrice;
    //     this.placeTime = placeTime;
    // }
}


