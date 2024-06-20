import BN from "bn.js";

export class PlayerInfo {
    name: string;
    secretKey?: Uint8Array;
}

export class PoolInfo {
    name: string;
    tokenName: string;
    mintDecimals: number;
    isStable: boolean;
}

export class TradeTokenInfo {
    name: string;
    discount: BN;
    liquidationFactor: BN;
}

export class MarketInfo {
    symbol: string;
    poolName: string;
    indexTokenName: string;
    stablePoolName: string;
}

export class ExchangeInitializeParams {
    poolInfos: PoolInfo[];
    playerInfos: PlayerInfo[];
    tradeTokenInfos: TradeTokenInfo[];
    marketInfos: MarketInfo[];


    constructor(poolInfos: PoolInfo[], playerInfos: PlayerInfo[], tradeTokenInfos: TradeTokenInfo[], marketInfos: MarketInfo[]) {
        this.poolInfos = poolInfos;
        this.playerInfos = playerInfos;
        this.tradeTokenInfos = tradeTokenInfos;
        this.marketInfos = marketInfos;
    }

    static defaultParams(): ExchangeInitializeParams {
        return new ExchangeInitializeParams(
            [
                {name: "BUMP_P__BTC", tokenName: "BTC", mintDecimals: 9, isStable: false},
                {name: "BUMP_P__SOL", tokenName: "SOL", mintDecimals: 9, isStable: false},
                {name: "BUMP_P__USDC", tokenName: "USDC", mintDecimals: 9, isStable: true}
            ],
            [
                {name: "Player1"},
                {name: "Player2"}
            ],
            [
                {name: "BTC", discount: new BN(1), liquidationFactor: new BN(1)},
                {name: "SOL", discount: new BN(1), liquidationFactor: new BN(1)},
                {name: "USDC", discount: new BN(1), liquidationFactor: new BN(1)}
            ],
            [
                {symbol: "BTCUSDC", poolName: "BUMP_P__BTC", indexTokenName: "BTC", stablePoolName: "BUMP_P__USDC"}
            ]
        );
    }
}