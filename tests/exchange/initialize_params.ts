export class PlayerInfo {
    name: string;
    secretKey?: Uint8Array;
}

export class PoolInfo {
    name: string;
    mintDecimals: number;
    isStable: boolean;
}

export class TradeTokenInfo {}

export class ExchangeInitializeParams {
    poolInfos: PoolInfo[];
    playerInfos: PlayerInfo[];


    constructor(poolInfos: PoolInfo[], playerInfos: PlayerInfo[]) {
        this.poolInfos = poolInfos;
        this.playerInfos = playerInfos;
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
            ]
        );
    }
}