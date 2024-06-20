export class OrderSide {
    static readonly NONE = {none: {}};
    static readonly LONG = {long: {}};
    static readonly SHORT = {short: {}};

}

export class OrderStatus {
    static readonly INIT = {init: {}};
    static readonly USING = {using: {}};
}

export class PositionSide {
    static readonly NONE = {none: {}};
    static readonly INCREASE = {increase: {}};
    static readonly DECREASE = {decrease: {}};
}

export class OrderType {
    static readonly NONE = {none: {}};
    static readonly MARKET = {market: {}};
    static readonly LIMIT = {limit: {}};
    static readonly STOP = {stop: {}};
}

export class StopType {
    static readonly NONE = {none: {}};
    static readonly StopLoss = {stopLoss: {}};
    static readonly TakeProfit = {takeProfit: {}};
}