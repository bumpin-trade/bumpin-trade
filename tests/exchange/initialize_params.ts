import BN from "bn.js";

export class PlayerInfo {
    name: string;
    secretKey?: Uint8Array;
}

export class PoolInfo {
    name: string;
    mintDecimals: number;
    isStable: boolean;
}

export class TradeTokenInfo {
    name: string;
    discount: BN;
    liquidationFactor: BN;
}

export class ExchangeInitializeParams {
    poolInfos: PoolInfo[];
    playerInfos: PlayerInfo[];
    tradeTokenInfos: TradeTokenInfo[];


    constructor(poolInfos: PoolInfo[], playerInfos: PlayerInfo[], tradeTokenInfos: TradeTokenInfo[]) {
        this.poolInfos = poolInfos;
        this.playerInfos = playerInfos;
        this.tradeTokenInfos = tradeTokenInfos;
    }

    static defaultParams(): ExchangeInitializeParams {
        return new ExchangeInitializeParams(
            [
                {name: "BUMP_P__BTC", mintDecimals: 9, isStable: false},
                {name: "BUMP_P__USDC", mintDecimals: 9, isStable: true}
            ],
            [
                {name: "Player1"},
                {name: "Player2"}
            ],
            [
                {name: "BTC", discount: new BN(1), liquidationFactor: new BN(1)},
                {name: "USDC", discount: new BN(1), liquidationFactor: new BN(1)}
            ]
        );
    }
}