/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/bumpin_trade.json`.
 */
export type BumpinTrade = {
    address: 'Ap5HaA55b1SrhMeBeiivgpbpA7ffTUtc64zcUJx7ionR';
    metadata: {
        name: 'bumpinTrade';
        version: '0.1.0';
        spec: '0.1.0';
        description: 'Created with Anchor';
    };
    instructions: [
        {
            name: 'addPositionMargin';
            discriminator: [52, 123, 95, 117, 1, 134, 241, 181];
            accounts: [
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'tradeToken';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.trade_token_index';
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePool';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.stable_pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'market';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [109, 97, 114, 107, 101, 116];
                            },
                            {
                                kind: 'arg';
                                path: 'params.market_index';
                            },
                        ];
                    };
                },
                {
                    name: 'poolMintVault';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'updatePositionMarginParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'adlCross';
            discriminator: [253, 180, 240, 79, 122, 82, 161, 237];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'arg';
                                path: 'params.user_authority_key';
                            },
                        ];
                    };
                },
                {
                    name: 'market';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [109, 97, 114, 107, 101, 116];
                            },
                            {
                                kind: 'arg';
                                path: 'params.market_index';
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.stable_pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePoolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.stable_pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeToken';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.trade_token_index';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeTokenVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.trade_token_index';
                            },
                        ];
                    };
                },
                {
                    name: 'keeperKey';
                    signer: true;
                    relations: ['state'];
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'adlParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'adlIsolate';
            discriminator: [67, 168, 137, 7, 149, 22, 242, 41];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'arg';
                                path: 'params.user_authority_key';
                            },
                        ];
                    };
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'market';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [109, 97, 114, 107, 101, 116];
                            },
                            {
                                kind: 'arg';
                                path: 'params.market_index';
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.stable_pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePoolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.stable_pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeToken';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.trade_token_index';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeTokenVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.trade_token_index';
                            },
                        ];
                    };
                },
                {
                    name: 'keeperKey';
                    signer: true;
                    relations: ['state'];
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'adlParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'autoCompound';
            discriminator: [190, 236, 229, 204, 126, 66, 94, 179];
            accounts: [
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'poolRewardsVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    114,
                                    101,
                                    119,
                                    97,
                                    114,
                                    100,
                                    115,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'poolIndex';
                    type: 'u16';
                },
            ];
        },
        {
            name: 'autoReblance';
            discriminator: [249, 239, 0, 145, 152, 147, 245, 63];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'stablePoolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePoolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'stablePoolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeToken';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'tradeTokenIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeTokenVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'tradeTokenIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'poolIndex';
                    type: 'u16';
                },
                {
                    name: 'stablePoolIndex';
                    type: 'u16';
                },
                {
                    name: 'tradeTokenIndex';
                    type: 'u16';
                },
            ];
        },
        {
            name: 'claimRewards';
            discriminator: [4, 144, 132, 71, 116, 23, 151, 80];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'rewards';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [114, 101, 119, 97, 114, 100, 115];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'poolRewardsVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    114,
                                    101,
                                    119,
                                    97,
                                    114,
                                    100,
                                    115,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'poolIndex';
                    type: 'u16';
                },
            ];
        },
        {
            name: 'collectRewards';
            discriminator: [63, 130, 90, 197, 39, 16, 143, 176];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePoolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'stablePoolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeToken';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'tradeTokenIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'stableTradeToken';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'stableTradeTokenIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'rewards';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [114, 101, 119, 97, 114, 100, 115];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'poolRewardsVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    114,
                                    101,
                                    119,
                                    97,
                                    114,
                                    100,
                                    115,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'daoRewardsVault';
                    writable: true;
                },
                {
                    name: 'keeperKey';
                    signer: true;
                    relations: ['state'];
                },
                {
                    name: 'bumpSigner';
                    relations: ['state'];
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'poolIndex';
                    type: 'u16';
                },
                {
                    name: 'stablePoolIndex';
                    type: 'u16';
                },
                {
                    name: 'tradeTokenIndex';
                    type: 'u16';
                },
                {
                    name: 'stableTradeTokenIndex';
                    type: 'u16';
                },
            ];
        },
        {
            name: 'deposit';
            discriminator: [242, 35, 198, 137, 82, 225, 242, 182];
            accounts: [
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'tradeToken';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'tokenIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeTokenVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'tokenIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'tokenIndex';
                    type: 'u16';
                },
                {
                    name: 'amount';
                    type: 'u128';
                },
            ];
        },
        {
            name: 'executePortfolioOrder';
            discriminator: [3, 115, 133, 59, 25, 49, 87, 214];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'arg';
                                path: 'params.user_authority_key';
                            },
                        ];
                    };
                },
                {
                    name: 'keeperKey';
                    signer: true;
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'executeOrderParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'executeWalletOrder';
            discriminator: [244, 115, 254, 183, 76, 71, 158, 40];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'arg';
                                path: 'params.user_authority_key';
                            },
                        ];
                    };
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'keeperKey';
                    signer: true;
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'executeOrderParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'initializeMarket';
            discriminator: [35, 35, 189, 193, 155, 48, 170, 203];
            accounts: [
                {
                    name: 'market';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [109, 97, 114, 107, 101, 116];
                            },
                            {
                                kind: 'account';
                                path: 'state.market_sequence';
                                account: 'state';
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.stable_pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'indexMintOracle';
                },
                {
                    name: 'admin';
                    writable: true;
                    signer: true;
                    relations: ['state'];
                },
                {
                    name: 'state';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'rent';
                    address: 'SysvarRent111111111111111111111111111111111';
                },
                {
                    name: 'systemProgram';
                    address: '11111111111111111111111111111111';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'initializeMarketParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'initializePool';
            discriminator: [95, 180, 10, 172, 84, 174, 232, 40];
            accounts: [
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'account';
                                path: 'state.pool_sequence';
                                account: 'state';
                            },
                        ];
                    };
                },
                {
                    name: 'poolMint';
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'account';
                                path: 'state.pool_sequence';
                                account: 'state';
                            },
                        ];
                    };
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'state';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'admin';
                    writable: true;
                    signer: true;
                    relations: ['state'];
                },
                {
                    name: 'rent';
                    address: 'SysvarRent111111111111111111111111111111111';
                },
                {
                    name: 'systemProgram';
                    address: '11111111111111111111111111111111';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'initializePoolParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'initializeRewards';
            discriminator: [91, 174, 112, 191, 233, 236, 147, 12];
            accounts: [
                {
                    name: 'state';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'poolMint';
                },
                {
                    name: 'rewards';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [114, 101, 119, 97, 114, 100, 115];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'poolRewardsVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    114,
                                    101,
                                    119,
                                    97,
                                    114,
                                    100,
                                    115,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'daoRewardsVault';
                },
                {
                    name: 'admin';
                    writable: true;
                    signer: true;
                    relations: ['state'];
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'rent';
                    address: 'SysvarRent111111111111111111111111111111111';
                },
                {
                    name: 'systemProgram';
                    address: '11111111111111111111111111111111';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'poolIndex';
                    type: 'u16';
                },
            ];
        },
        {
            name: 'initializeState';
            discriminator: [190, 171, 224, 219, 217, 72, 199, 176];
            accounts: [
                {
                    name: 'admin';
                    writable: true;
                    signer: true;
                },
                {
                    name: 'state';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'rent';
                    address: 'SysvarRent111111111111111111111111111111111';
                },
                {
                    name: 'systemProgram';
                    address: '11111111111111111111111111111111';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'param';
                    type: {
                        defined: {
                            name: 'initializeStateParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'initializeTradeToken';
            discriminator: [188, 220, 217, 110, 223, 180, 96, 121];
            accounts: [
                {
                    name: 'tradeToken';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                ];
                            },
                            {
                                kind: 'account';
                                path: 'state.trade_token_sequence';
                                account: 'state';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeTokenMint';
                },
                {
                    name: 'tradeTokenVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'account';
                                path: 'state.trade_token_sequence';
                                account: 'state';
                            },
                        ];
                    };
                },
                {
                    name: 'oracle';
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'state';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'admin';
                    writable: true;
                    signer: true;
                    relations: ['state'];
                },
                {
                    name: 'rent';
                    address: 'SysvarRent111111111111111111111111111111111';
                },
                {
                    name: 'systemProgram';
                    address: '11111111111111111111111111111111';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'discount';
                    type: 'u32';
                },
                {
                    name: 'mintName';
                    type: {
                        array: ['u8', 32];
                    };
                },
                {
                    name: 'liquidationFactor';
                    type: 'u32';
                },
            ];
        },
        {
            name: 'initializeUser';
            discriminator: [111, 17, 185, 250, 60, 122, 38, 254];
            accounts: [
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'payer';
                    writable: true;
                    signer: true;
                },
                {
                    name: 'rent';
                    address: 'SysvarRent111111111111111111111111111111111';
                },
                {
                    name: 'systemProgram';
                    address: '11111111111111111111111111111111';
                },
            ];
            args: [];
        },
        {
            name: 'liquidateCrossPosition';
            discriminator: [40, 173, 153, 195, 116, 68, 144, 117];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'arg';
                                path: 'userAuthorityKey';
                            },
                        ];
                    };
                },
                {
                    name: 'keeperKey';
                    signer: true;
                    relations: ['state'];
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'userAuthorityKey';
                    type: 'pubkey';
                },
            ];
        },
        {
            name: 'liquidateIsolatePosition';
            discriminator: [88, 101, 146, 105, 53, 188, 251, 89];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'arg';
                                path: 'params.user_authority_key';
                            },
                        ];
                    };
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'market';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [109, 97, 114, 107, 101, 116];
                            },
                            {
                                kind: 'arg';
                                path: 'params.market_index';
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.stable_pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePoolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.stable_pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeToken';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.trade_token_index';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeTokenVault';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.trade_token_index';
                            },
                        ];
                    };
                },
                {
                    name: 'keeperKey';
                    signer: true;
                    relations: ['state'];
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'liquidateIsolatePositionParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'modifyState';
            discriminator: [138, 33, 5, 32, 222, 139, 151, 15];
            accounts: [
                {
                    name: 'state';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'admin';
                    signer: true;
                    relations: ['state'];
                },
            ];
            args: [
                {
                    name: 'param';
                    type: {
                        defined: {
                            name: 'modifyStateParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'placePortfolioOrder';
            discriminator: [151, 96, 202, 84, 56, 114, 103, 11];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'order';
                    type: {
                        defined: {
                            name: 'placeOrderParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'placeWalletOrder';
            discriminator: [135, 59, 183, 1, 114, 135, 190, 126];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'order';
                    type: {
                        defined: {
                            name: 'placeOrderParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'portfolioCancelOrder';
            discriminator: [129, 135, 63, 188, 5, 127, 249, 138];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'pool';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'cancelOrderParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'portfolioStake';
            discriminator: [197, 248, 167, 245, 97, 196, 252, 19];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeTokenVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'tradeTokenIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'poolIndex';
                    type: 'u16';
                },
                {
                    name: 'tradeTokenIndex';
                    type: 'u16';
                },
                {
                    name: 'requestTokenAmount';
                    type: 'u128';
                },
            ];
        },
        {
            name: 'portfolioUnStake';
            discriminator: [38, 230, 213, 92, 0, 48, 190, 42];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeTokenVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.trade_token_index';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeToken';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.trade_token_index';
                            },
                        ];
                    };
                },
                {
                    name: 'poolRewardsVault';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    114,
                                    101,
                                    119,
                                    97,
                                    114,
                                    100,
                                    115,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'unStakeParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'updatePositionLeverage';
            discriminator: [193, 183, 118, 54, 175, 135, 124, 132];
            accounts: [
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'stablePool';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'market';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [109, 97, 114, 107, 101, 116];
                            },
                            {
                                kind: 'arg';
                                path: 'params.market_index';
                            },
                        ];
                    };
                },
                {
                    name: 'poolMintVault';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'updatePositionLeverageParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'walletCancelOrder';
            discriminator: [88, 166, 154, 153, 63, 80, 209, 200];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'pool';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'cancelOrderParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'walletStake';
            discriminator: [133, 206, 14, 95, 182, 13, 19, 134];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'poolIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'poolIndex';
                    type: 'u16';
                },
                {
                    name: 'requestTokenAmount';
                    type: 'u128';
                },
            ];
        },
        {
            name: 'walletUnStake';
            discriminator: [233, 75, 110, 151, 143, 26, 47, 229];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'pool';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [112, 111, 111, 108];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'poolVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'tradeToken';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.trade_token_index';
                            },
                        ];
                    };
                },
                {
                    name: 'poolRewardsVault';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    112,
                                    111,
                                    111,
                                    108,
                                    95,
                                    114,
                                    101,
                                    119,
                                    97,
                                    114,
                                    100,
                                    115,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'params.pool_index';
                            },
                        ];
                    };
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'unStakeParams';
                        };
                    };
                },
            ];
        },
        {
            name: 'withdraw';
            discriminator: [183, 18, 70, 156, 148, 109, 161, 34];
            accounts: [
                {
                    name: 'state';
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    98,
                                    117,
                                    109,
                                    112,
                                    95,
                                    115,
                                    116,
                                    97,
                                    116,
                                    101,
                                ];
                            },
                        ];
                    };
                },
                {
                    name: 'user';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [117, 115, 101, 114];
                            },
                            {
                                kind: 'account';
                                path: 'authority';
                            },
                        ];
                    };
                },
                {
                    name: 'authority';
                    signer: true;
                },
                {
                    name: 'userTokenAccount';
                    writable: true;
                },
                {
                    name: 'tradeTokenVault';
                    writable: true;
                    pda: {
                        seeds: [
                            {
                                kind: 'const';
                                value: [
                                    116,
                                    114,
                                    97,
                                    100,
                                    101,
                                    95,
                                    116,
                                    111,
                                    107,
                                    101,
                                    110,
                                    95,
                                    118,
                                    97,
                                    117,
                                    108,
                                    116,
                                ];
                            },
                            {
                                kind: 'arg';
                                path: 'tokenIndex';
                            },
                        ];
                    };
                },
                {
                    name: 'bumpSigner';
                },
                {
                    name: 'tokenProgram';
                    address: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA';
                },
            ];
            args: [
                {
                    name: 'tokenIndex';
                    type: 'u16';
                },
                {
                    name: 'amount';
                    type: 'u128';
                },
            ];
        },
    ];
    accounts: [
        {
            name: 'market';
            discriminator: [219, 190, 213, 55, 0, 227, 198, 154];
        },
        {
            name: 'pool';
            discriminator: [241, 154, 109, 4, 17, 177, 109, 188];
        },
        {
            name: 'rewards';
            discriminator: [12, 223, 68, 101, 63, 33, 38, 101];
        },
        {
            name: 'state';
            discriminator: [216, 146, 107, 94, 104, 75, 182, 177];
        },
        {
            name: 'tradeToken';
            discriminator: [228, 70, 239, 205, 241, 218, 48, 76];
        },
        {
            name: 'user';
            discriminator: [159, 117, 95, 227, 239, 151, 58, 236];
        },
    ];
    events: [
        {
            name: 'addOrDecreaseMarginEvent';
            discriminator: [255, 34, 9, 229, 17, 100, 24, 85];
        },
        {
            name: 'addOrDeleteUserOrderEvent';
            discriminator: [226, 197, 6, 41, 154, 69, 25, 217];
        },
        {
            name: 'addOrDeleteUserPositionEvent';
            discriminator: [62, 52, 185, 169, 23, 84, 198, 48];
        },
        {
            name: 'depositEvent';
            discriminator: [120, 248, 61, 83, 31, 142, 107, 144];
        },
        {
            name: 'initUserEvent';
            discriminator: [172, 69, 161, 169, 238, 167, 121, 162];
        },
        {
            name: 'poolUpdateEvent';
            discriminator: [124, 213, 52, 182, 189, 225, 61, 254];
        },
        {
            name: 'stakeOrUnStakeEvent';
            discriminator: [80, 62, 133, 137, 171, 83, 189, 214];
        },
        {
            name: 'updateUserPositionEvent';
            discriminator: [102, 98, 41, 238, 50, 237, 214, 100];
        },
        {
            name: 'userHoldUpdateEvent';
            discriminator: [134, 142, 93, 85, 54, 61, 131, 93];
        },
        {
            name: 'userRewardsUpdateEvent';
            discriminator: [93, 62, 28, 230, 211, 53, 192, 119];
        },
        {
            name: 'userTokenBalanceUpdateEvent';
            discriminator: [6, 46, 145, 182, 47, 123, 79, 108];
        },
        {
            name: 'withdrawEvent';
            discriminator: [22, 9, 133, 26, 160, 44, 71, 192];
        },
    ];
    errors: [
        {
            code: 6000;
            name: 'amountNotEnough';
            msg: 'amountNotEnough';
        },
        {
            code: 6001;
            name: 'subHoldPoolBiggerThanHold';
            msg: 'subHoldPoolBiggerThanHold';
        },
        {
            code: 6002;
            name: 'subPoolStableAmountBiggerThanStableAmount';
            msg: 'subPoolStableAmountBiggerThanStableAmount';
        },
        {
            code: 6003;
            name: 'subPoolAmountBiggerThanAmount';
            msg: 'subPoolAmountBiggerThanAmount';
        },
        {
            code: 6004;
            name: 'positionShouldBeLiquidation';
            msg: 'positionShouldBeLiquidation';
        },
        {
            code: 6005;
            name: 'orderHoldUsdSmallThanHoldUsd';
            msg: 'orderHoldUsdSmallThanHoldUsd';
        },
        {
            code: 6006;
            name: 'standardPoolValueNotEnough';
            msg: 'standardPoolValueNotEnough';
        },
        {
            code: 6007;
            name: 'orderMarginUsdTooSmall';
            msg: 'orderMarginUsdTooSmall';
        },
        {
            code: 6008;
            name: 'poolAvailableLiquidityNotEnough';
            msg: 'poolAvailableLiquidityNotEnough';
        },
        {
            code: 6009;
            name: 'invalidTransfer';
            msg: 'Invalid transfer';
        },
        {
            code: 6010;
            name: 'invalidParam';
            msg: 'invalidParam';
        },
        {
            code: 6011;
            name: 'onlyOneTypeOrderAllowed';
            msg: 'onlyOneTypeOrderAllowed';
        },
        {
            code: 6012;
            name: 'orderNotExist';
            msg: 'orderNotExist';
        },
        {
            code: 6013;
            name: 'tokenNotMatch';
            msg: 'tokenNotMatch';
        },
        {
            code: 6014;
            name: 'noMoreUserTokenSpace';
            msg: 'noMoreUserTokenSpace';
        },
        {
            code: 6015;
            name: 'noMoreOrderSpace';
            msg: 'noMoreOrderSpace';
        },
        {
            code: 6016;
            name: 'leverageIsNotAllowed';
            msg: 'leverageIsNotAllowed';
        },
        {
            code: 6017;
            name: 'priceIsNotAllowed';
            msg: 'priceIsNotAllowed';
        },
        {
            code: 6018;
            name: 'onlyOneDirectionPositionIsAllowed';
            msg: 'onlyOneDirectionPositionIsAllowed';
        },
        {
            code: 6019;
            name: 'balanceNotEnough';
            msg: 'balanceNotEnough';
        },
        {
            code: 6020;
            name: 'pythOffline';
            msg: 'pythOffline';
        },
        {
            code: 6021;
            name: 'overflow';
            msg: 'overflow';
        },
        {
            code: 6022;
            name: 'transferFailed';
            msg: 'transferFailed';
        },
        {
            code: 6023;
            name: 'unableToLoadAccountLoader';
            msg: 'Unable to load AccountLoader';
        },
        {
            code: 6024;
            name: 'cantPayUserInitFee';
            msg: 'cantPayUserInitFee';
        },
        {
            code: 6025;
            name: 'couldNotFindUserToken';
            msg: 'couldNotFindUserToken';
        },
        {
            code: 6026;
            name: 'couldNotFindUserOrder';
            msg: 'couldNotFindUserOrder';
        },
        {
            code: 6027;
            name: 'couldNotFindUserPosition';
            msg: 'couldNotFindUserPosition';
        },
        {
            code: 6028;
            name: 'liquidatePositionIgnore';
            msg: 'liquidatePositionIgnore';
        },
        {
            code: 6029;
            name: 'onlyCrossPositionAllowed';
            msg: 'onlyCrossPositionAllowed';
        },
        {
            code: 6030;
            name: 'onlyIsolatePositionAllowed';
            msg: 'onlyIsolatePositionAllowed';
        },
        {
            code: 6031;
            name: 'couldNotFindUserStake';
            msg: 'couldNotFindUserStake';
        },
        {
            code: 6032;
            name: 'oracleNotFound';
            msg: 'oracleNotFound';
        },
        {
            code: 6033;
            name: 'oraclePriceToOld';
            msg: 'oraclePriceToOld';
        },
        {
            code: 6034;
            name: 'unableToLoadOracle';
            msg: 'Unable To Load Oracles';
        },
        {
            code: 6035;
            name: 'invalidOracle';
            msg: 'invalidOracle';
        },
        {
            code: 6036;
            name: 'bnConversionError';
            msg: 'Conversion to u128/u128 failed with an overflow or underflow';
        },
        {
            code: 6037;
            name: 'mathError';
            msg: 'Math Error';
        },
        {
            code: 6038;
            name: 'castingFailure';
            msg: 'Casting Failure';
        },
        {
            code: 6039;
            name: 'couldNotLoadMarketData';
            msg: 'couldNotLoadMarketData';
        },
        {
            code: 6040;
            name: 'couldNotFindMarket';
            msg: 'couldNotFindMarket';
        },
        {
            code: 6041;
            name: 'invalidMarketAccount';
            msg: 'invalidMarketAccount';
        },
        {
            code: 6042;
            name: 'marketWrongMutability';
            msg: 'marketWrongMutability';
        },
        {
            code: 6043;
            name: 'marketNumberNotEqual2Pool';
            msg: 'marketNumberNotEqual2Pool';
        },
        {
            code: 6044;
            name: 'failedUnwrap';
            msg: 'Failed Unwrap';
        },
        {
            code: 6045;
            name: 'userNotEnoughValue';
            msg: 'User Not Enough Value';
        },
        {
            code: 6046;
            name: 'amountZero';
            msg: 'amountZero';
        },
        {
            code: 6047;
            name: 'couldNotLoadTokenAccountData';
            msg: 'couldNotLoadTokenAccountData';
        },
        {
            code: 6048;
            name: 'couldNotLoadTradeTokenData';
            msg: 'couldNotLoadTradeTokenData';
        },
        {
            code: 6049;
            name: 'couldNotLoadPoolData';
            msg: 'couldNotLoadPoolData';
        },
        {
            code: 6050;
            name: 'invalidTradeTokenAccount';
            msg: 'invalidTradeTokenAccount';
        },
        {
            code: 6051;
            name: 'invalidTokenAccount';
            msg: 'invalidTokenAccount';
        },
        {
            code: 6052;
            name: 'invalidPoolAccount';
            msg: 'invalidPoolAccount';
        },
        {
            code: 6053;
            name: 'tradeTokenNotFind';
            msg: 'canNotFindTradeToken';
        },
        {
            code: 6054;
            name: 'vaultNotFind';
            msg: 'canNotFindVault';
        },
        {
            code: 6055;
            name: 'marketNotFind';
            msg: 'canNotFindMarket';
        },
        {
            code: 6056;
            name: 'stakePaused';
            msg: 'stakePaused';
        },
        {
            code: 6057;
            name: 'stakeToSmall';
            msg: 'stakeToSmall';
        },
        {
            code: 6058;
            name: 'unStakeTooSmall';
            msg: 'unStakeTooSmall';
        },
        {
            code: 6059;
            name: 'unStakeWithAmountNotEnough';
            msg: 'unStakeWithAmountNotEnough';
        },
        {
            code: 6060;
            name: 'unStakeTooLarge';
            msg: 'unStakeTooLarge';
        },
        {
            code: 6061;
            name: 'positionSideNotSupport';
            msg: 'positionSideNotSupport';
        },
        {
            code: 6062;
            name: 'rewardsNotFound';
            msg: 'rewardsNotFound';
        },
        {
            code: 6063;
            name: 'userNotFound';
            msg: 'userNotFound';
        },
        {
            code: 6064;
            name: 'couldNotLoadUserData';
            msg: 'couldNotLoadUserData';
        },
        {
            code: 6065;
            name: 'poolSubUnsettleNotEnough';
            msg: 'poolSubUnsettleNotEnough';
        },
        {
            code: 6066;
            name: 'timestampNotFound';
            msg: 'timestampNotFound';
        },
        {
            code: 6067;
            name: 'claimUnqualified';
            msg: 'claimUnqualified';
        },
        {
            code: 6068;
            name: 'poolMintSupplyIsZero';
            msg: 'poolMintSupplyIsZero';
        },
    ];
    types: [
        {
            name: 'adlParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'poolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'stablePoolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'marketIndex';
                        type: 'u16';
                    },
                    {
                        name: 'tradeTokenIndex';
                        type: 'u16';
                    },
                    {
                        name: 'positionKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'userAuthorityKey';
                        type: 'pubkey';
                    },
                ];
            };
        },
        {
            name: 'addOrDecreaseMarginEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'userKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'position';
                        type: {
                            defined: {
                                name: 'userPosition';
                            };
                        };
                    },
                    {
                        name: 'prePosition';
                        type: {
                            defined: {
                                name: 'userPosition';
                            };
                        };
                    },
                    {
                        name: 'isAdd';
                        type: 'bool';
                    },
                ];
            };
        },
        {
            name: 'addOrDeleteUserOrderEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'userKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'order';
                        type: {
                            defined: {
                                name: 'userOrder';
                            };
                        };
                    },
                    {
                        name: 'isAdd';
                        type: 'bool';
                    },
                ];
            };
        },
        {
            name: 'addOrDeleteUserPositionEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'position';
                        type: {
                            defined: {
                                name: 'userPosition';
                            };
                        };
                    },
                    {
                        name: 'isAdd';
                        type: 'bool';
                    },
                ];
            };
        },
        {
            name: 'borrowingFee';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'totalBorrowingFee';
                        type: 'u128';
                    },
                    {
                        name: 'totalRealizedBorrowingFee';
                        type: 'u128';
                    },
                    {
                        name: 'cumulativeBorrowingFeePerToken';
                        type: 'u128';
                    },
                    {
                        name: 'updatedAt';
                        type: 'i64';
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 8];
                        };
                    },
                ];
            };
        },
        {
            name: 'cancelOrderParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'poolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'orderId';
                        type: 'u64';
                    },
                ];
            };
        },
        {
            name: 'depositEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'userKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'tokenMint';
                        type: 'pubkey';
                    },
                    {
                        name: 'amount';
                        type: 'u128';
                    },
                    {
                        name: 'depositOrigin';
                        type: {
                            defined: {
                                name: 'depositOrigin';
                            };
                        };
                    },
                ];
            };
        },
        {
            name: 'depositOrigin';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'manual';
                    },
                    {
                        name: 'order';
                    },
                    {
                        name: 'stake';
                    },
                ];
            };
        },
        {
            name: 'executeOrderParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'orderId';
                        type: 'u64';
                    },
                    {
                        name: 'userAuthorityKey';
                        type: 'pubkey';
                    },
                ];
            };
        },
        {
            name: 'feeReward';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'feeAmount';
                        docs: [
                            'Fees generated from staking, redeeming, and position operations (such as increasing or decreasing positions).',
                            'Used in isolated margin mode (since fees are actually transferred each time they are generated).',
                            'See: [`fee_processor::collect_long_open_position_fee`]',
                        ];
                        type: 'u128';
                    },
                    {
                        name: 'unSettleFeeAmount';
                        docs: [
                            'Accounting for fees generated from cross-margin position operations.',
                        ];
                        type: 'u128';
                    },
                    {
                        name: 'cumulativeRewardsPerStakeToken';
                        docs: [
                            'Cursor that records how many tokens each stake share can receive.',
                            'This increases during keeper collect operations.',
                            'Users can calculate their rewards based on this cursor and the corresponding field in their UserStake.',
                        ];
                        type: 'u128';
                    },
                    {
                        name: 'lastRewardsPerStakeTokenDeltas';
                        docs: [
                            'Records the deltas of the last three keeper collect operations.',
                            'Each time the keeper calls collect, a delta is recorded here.',
                            'When distributing rewards to users, it must be determined if the user has experienced a sufficiently long period (i.e., three keeper calls).',
                        ];
                        type: {
                            array: ['u128', 3];
                        };
                    },
                ];
            };
        },
        {
            name: 'initUserEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'userKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'authority';
                        type: 'pubkey';
                    },
                ];
            };
        },
        {
            name: 'initializeMarketParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'symbol';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'tickSize';
                        type: 'u128';
                    },
                    {
                        name: 'openFeeRate';
                        type: 'u128';
                    },
                    {
                        name: 'closeFeeRate';
                        type: 'u128';
                    },
                    {
                        name: 'maximumLongOpenInterestCap';
                        type: 'u128';
                    },
                    {
                        name: 'maximumShortOpenInterestCap';
                        type: 'u128';
                    },
                    {
                        name: 'longShortRatioLimit';
                        type: 'u128';
                    },
                    {
                        name: 'longShortOiBottomLimit';
                        type: 'u128';
                    },
                    {
                        name: 'maximumLeverage';
                        type: 'u32';
                    },
                    {
                        name: 'minimumLeverage';
                        type: 'u32';
                    },
                    {
                        name: 'poolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'stablePoolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'maxPoolLiquidityShareRate';
                        type: 'u32';
                    },
                ];
            };
        },
        {
            name: 'initializePoolParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'name';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'stableMintKey';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'poolConfig';
                        type: {
                            defined: {
                                name: 'poolConfig';
                            };
                        };
                    },
                    {
                        name: 'stable';
                        type: 'bool';
                    },
                ];
            };
        },
        {
            name: 'initializeStateParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'keeperKey';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'minOrderMarginUsd';
                        type: 'u128';
                    },
                    {
                        name: 'maximumMaintenanceMarginRate';
                        type: 'u32';
                    },
                    {
                        name: 'fundingFeeBaseRate';
                        type: 'u128';
                    },
                    {
                        name: 'maxFundingBaseRate';
                        type: 'u128';
                    },
                    {
                        name: 'tradingFeeStakingRewardsRatio';
                        type: 'u128';
                    },
                    {
                        name: 'tradingFeePoolRewardsRatio';
                        type: 'u128';
                    },
                    {
                        name: 'tradingFeeUsdPoolRewardsRatio';
                        type: 'u128';
                    },
                    {
                        name: 'borrowingFeeStakingRewardsRatio';
                        type: 'u128';
                    },
                    {
                        name: 'borrowingFeePoolRewardsRatio';
                        type: 'u128';
                    },
                    {
                        name: 'minPrecisionMultiple';
                        type: 'u128';
                    },
                    {
                        name: 'mintFeeStakingRewardsRatio';
                        type: 'u128';
                    },
                    {
                        name: 'mintFeePoolRewardsRatio';
                        type: 'u128';
                    },
                    {
                        name: 'redeemFeeStakingRewardsRatio';
                        type: 'u128';
                    },
                    {
                        name: 'redeemFeePoolRewardsRatio';
                        type: 'u128';
                    },
                    {
                        name: 'poolRewardsIntervalLimit';
                        type: 'u128';
                    },
                    {
                        name: 'initFee';
                        type: 'u64';
                    },
                    {
                        name: 'stakingFeeRewardRatio';
                        type: 'u32';
                    },
                    {
                        name: 'poolFeeRewardRatio';
                        type: 'u32';
                    },
                ];
            };
        },
        {
            name: 'liquidateIsolatePositionParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'positionKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'marketIndex';
                        type: 'u16';
                    },
                    {
                        name: 'tradeTokenIndex';
                        type: 'u16';
                    },
                    {
                        name: 'poolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'stablePoolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'userAuthorityKey';
                        type: 'pubkey';
                    },
                ];
            };
        },
        {
            name: 'market';
            serialization: 'bytemuckunsafe';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'symbol';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'longOpenInterest';
                        type: {
                            defined: {
                                name: 'marketPosition';
                            };
                        };
                    },
                    {
                        name: 'shortOpenInterest';
                        type: {
                            defined: {
                                name: 'marketPosition';
                            };
                        };
                    },
                    {
                        name: 'fundingFee';
                        type: {
                            defined: {
                                name: 'marketFundingFee';
                            };
                        };
                    },
                    {
                        name: 'config';
                        type: {
                            defined: {
                                name: 'marketConfig';
                            };
                        };
                    },
                    {
                        name: 'poolKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'poolMintKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'indexMintOracle';
                        type: 'pubkey';
                    },
                    {
                        name: 'stablePoolKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'stablePoolMintKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'index';
                        type: 'u16';
                    },
                    {
                        name: 'marketStatus';
                        type: {
                            defined: {
                                name: 'marketStatus';
                            };
                        };
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 13];
                        };
                    },
                    {
                        name: 'reservePadding';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                ];
            };
        },
        {
            name: 'marketConfig';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'tickSize';
                        type: 'u128';
                    },
                    {
                        name: 'openFeeRate';
                        type: 'u128';
                    },
                    {
                        name: 'closeFeeRate';
                        type: 'u128';
                    },
                    {
                        name: 'maximumLongOpenInterestCap';
                        type: 'u128';
                    },
                    {
                        name: 'maximumShortOpenInterestCap';
                        type: 'u128';
                    },
                    {
                        name: 'longShortRatioLimit';
                        type: 'u128';
                    },
                    {
                        name: 'longShortOiBottomLimit';
                        type: 'u128';
                    },
                    {
                        name: 'maximumLeverage';
                        type: 'u32';
                    },
                    {
                        name: 'minimumLeverage';
                        type: 'u32';
                    },
                    {
                        name: 'maxPoolLiquidityShareRate';
                        type: 'u32';
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 4];
                        };
                    },
                ];
            };
        },
        {
            name: 'marketFundingFee';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'longFundingFeeAmountPerSize';
                        type: 'i128';
                    },
                    {
                        name: 'shortFundingFeeAmountPerSize';
                        type: 'i128';
                    },
                    {
                        name: 'totalLongFundingFee';
                        type: 'i128';
                    },
                    {
                        name: 'totalShortFundingFee';
                        type: 'i128';
                    },
                    {
                        name: 'longFundingFeeRate';
                        type: 'i128';
                    },
                    {
                        name: 'shortFundingFeeRate';
                        type: 'i128';
                    },
                    {
                        name: 'updatedAt';
                        type: 'i64';
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 8];
                        };
                    },
                ];
            };
        },
        {
            name: 'marketPosition';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'openInterest';
                        type: 'u128';
                    },
                    {
                        name: 'entryPrice';
                        type: 'u128';
                    },
                ];
            };
        },
        {
            name: 'marketStatus';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'normal';
                    },
                    {
                        name: 'reduceOnly';
                    },
                    {
                        name: 'pause';
                    },
                ];
            };
        },
        {
            name: 'modifyStateParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'minOrderMarginUsd';
                        type: {
                            option: 'u128';
                        };
                    },
                    {
                        name: 'maximumMaintenanceMarginRate';
                        type: {
                            option: 'u32';
                        };
                    },
                    {
                        name: 'fundingFeeBaseRate';
                        type: {
                            option: 'u128';
                        };
                    },
                    {
                        name: 'maxFundingBaseRate';
                        type: {
                            option: 'u128';
                        };
                    },
                    {
                        name: 'tradingFeeStakingRewardsRatio';
                        type: {
                            option: 'u32';
                        };
                    },
                    {
                        name: 'tradingFeePoolRewardsRatio';
                        type: {
                            option: 'u32';
                        };
                    },
                    {
                        name: 'tradingFeeUsdPoolRewardsRatio';
                        type: {
                            option: 'u32';
                        };
                    },
                    {
                        name: 'minPrecisionMultiple';
                        type: {
                            option: 'u128';
                        };
                    },
                    {
                        name: 'poolRewardsIntervalLimit';
                        type: {
                            option: 'u128';
                        };
                    },
                    {
                        name: 'initFee';
                        type: {
                            option: 'u64';
                        };
                    },
                    {
                        name: 'stakingFeeRewardRatio';
                        type: {
                            option: 'u32';
                        };
                    },
                    {
                        name: 'poolFeeRewardRatio';
                        type: {
                            option: 'u32';
                        };
                    },
                    {
                        name: 'essentialAccountAlt';
                        type: {
                            option: {
                                array: ['u8', 32];
                            };
                        };
                    },
                ];
            };
        },
        {
            name: 'orderSide';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'none';
                    },
                    {
                        name: 'long';
                    },
                    {
                        name: 'short';
                    },
                ];
            };
        },
        {
            name: 'orderStatus';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'init';
                    },
                    {
                        name: 'using';
                    },
                ];
            };
        },
        {
            name: 'orderType';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'none';
                    },
                    {
                        name: 'market';
                    },
                    {
                        name: 'limit';
                    },
                    {
                        name: 'stop';
                    },
                ];
            };
        },
        {
            name: 'placeOrderParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'symbol';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'size';
                        type: 'u128';
                    },
                    {
                        name: 'orderMargin';
                        type: 'u128';
                    },
                    {
                        name: 'leverage';
                        type: 'u32';
                    },
                    {
                        name: 'triggerPrice';
                        type: 'u128';
                    },
                    {
                        name: 'acceptablePrice';
                        type: 'u128';
                    },
                    {
                        name: 'placeTime';
                        type: 'i64';
                    },
                    {
                        name: 'isPortfolioMargin';
                        type: 'bool';
                    },
                    {
                        name: 'isNativeToken';
                        type: 'bool';
                    },
                    {
                        name: 'orderSide';
                        type: {
                            defined: {
                                name: 'orderSide';
                            };
                        };
                    },
                    {
                        name: 'positionSide';
                        type: {
                            defined: {
                                name: 'positionSide';
                            };
                        };
                    },
                    {
                        name: 'orderType';
                        type: {
                            defined: {
                                name: 'orderType';
                            };
                        };
                    },
                    {
                        name: 'stopType';
                        type: {
                            defined: {
                                name: 'stopType';
                            };
                        };
                    },
                ];
            };
        },
        {
            name: 'pool';
            serialization: 'bytemuckunsafe';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'key';
                        type: 'pubkey';
                    },
                    {
                        name: 'name';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'pnl';
                        type: 'i128';
                    },
                    {
                        name: 'apr';
                        type: 'u128';
                    },
                    {
                        name: 'insuranceFundAmount';
                        type: 'u128';
                    },
                    {
                        name: 'totalSupply';
                        type: 'u128';
                    },
                    {
                        name: 'balance';
                        type: {
                            defined: {
                                name: 'poolBalance';
                            };
                        };
                    },
                    {
                        name: 'stableBalance';
                        type: {
                            defined: {
                                name: 'poolBalance';
                            };
                        };
                    },
                    {
                        name: 'borrowingFee';
                        type: {
                            defined: {
                                name: 'borrowingFee';
                            };
                        };
                    },
                    {
                        name: 'feeReward';
                        type: {
                            defined: {
                                name: 'feeReward';
                            };
                        };
                    },
                    {
                        name: 'stableFeeReward';
                        type: {
                            defined: {
                                name: 'feeReward';
                            };
                        };
                    },
                    {
                        name: 'config';
                        type: {
                            defined: {
                                name: 'poolConfig';
                            };
                        };
                    },
                    {
                        name: 'poolVaultKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'stableMintKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'mintKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'index';
                        type: 'u16';
                    },
                    {
                        name: 'status';
                        type: {
                            defined: {
                                name: 'poolStatus';
                            };
                        };
                    },
                    {
                        name: 'stable';
                        type: 'bool';
                    },
                    {
                        name: 'marketNumber';
                        type: 'u16';
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 8];
                        };
                    },
                    {
                        name: 'reservePadding';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                ];
            };
        },
        {
            name: 'poolBalance';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'settleFundingFee';
                        type: 'i128';
                    },
                    {
                        name: 'amount';
                        type: 'u128';
                    },
                    {
                        name: 'holdAmount';
                        type: 'u128';
                    },
                    {
                        name: 'unSettleAmount';
                        type: 'u128';
                    },
                    {
                        name: 'lossAmount';
                        type: 'u128';
                    },
                ];
            };
        },
        {
            name: 'poolConfig';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'minimumStakeAmount';
                        type: 'u128';
                    },
                    {
                        name: 'minimumUnStakeAmount';
                        type: 'u128';
                    },
                    {
                        name: 'poolLiquidityLimit';
                        type: 'u128';
                    },
                    {
                        name: 'borrowingInterestRate';
                        type: 'u128';
                    },
                    {
                        name: 'stakeFeeRate';
                        type: 'u32';
                    },
                    {
                        name: 'unStakeFeeRate';
                        type: 'u32';
                    },
                    {
                        name: 'unSettleMintRatioLimit';
                        type: 'u32';
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 4];
                        };
                    },
                ];
            };
        },
        {
            name: 'poolStatus';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'normal';
                    },
                    {
                        name: 'stakePaused';
                    },
                    {
                        name: 'unStakePaused';
                    },
                ];
            };
        },
        {
            name: 'poolUpdateEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'poolKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'poolMint';
                        type: 'pubkey';
                    },
                    {
                        name: 'poolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'poolBalance';
                        type: {
                            defined: {
                                name: 'poolBalance';
                            };
                        };
                    },
                    {
                        name: 'stableBalance';
                        type: {
                            defined: {
                                name: 'poolBalance';
                            };
                        };
                    },
                    {
                        name: 'borrowingFee';
                        type: {
                            defined: {
                                name: 'borrowingFee';
                            };
                        };
                    },
                    {
                        name: 'feeReward';
                        type: {
                            defined: {
                                name: 'feeReward';
                            };
                        };
                    },
                    {
                        name: 'stableFeeReward';
                        type: {
                            defined: {
                                name: 'feeReward';
                            };
                        };
                    },
                    {
                        name: 'totalSupply';
                        type: 'u128';
                    },
                    {
                        name: 'pnl';
                        type: 'i128';
                    },
                    {
                        name: 'apr';
                        type: 'u128';
                    },
                    {
                        name: 'insuranceFundAmount';
                        type: 'u128';
                    },
                    {
                        name: 'prePoolBalance';
                        type: {
                            defined: {
                                name: 'poolBalance';
                            };
                        };
                    },
                    {
                        name: 'preStableBalance';
                        type: {
                            defined: {
                                name: 'poolBalance';
                            };
                        };
                    },
                    {
                        name: 'preBorrowingFee';
                        type: {
                            defined: {
                                name: 'borrowingFee';
                            };
                        };
                    },
                    {
                        name: 'preFeeReward';
                        type: {
                            defined: {
                                name: 'feeReward';
                            };
                        };
                    },
                    {
                        name: 'preStableFeeReward';
                        type: {
                            defined: {
                                name: 'feeReward';
                            };
                        };
                    },
                    {
                        name: 'preTotalSupply';
                        type: 'u128';
                    },
                    {
                        name: 'prePnl';
                        type: 'i128';
                    },
                    {
                        name: 'preApr';
                        type: 'u128';
                    },
                    {
                        name: 'preInsuranceFundAmount';
                        type: 'u128';
                    },
                ];
            };
        },
        {
            name: 'positionSide';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'none';
                    },
                    {
                        name: 'increase';
                    },
                    {
                        name: 'decrease';
                    },
                ];
            };
        },
        {
            name: 'positionStatus';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'init';
                    },
                    {
                        name: 'using';
                    },
                ];
            };
        },
        {
            name: 'rewards';
            serialization: 'bytemuckunsafe';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'poolUnClaimAmount';
                        type: 'u128';
                    },
                    {
                        name: 'poolTotalRewardsAmount';
                        type: 'u128';
                    },
                    {
                        name: 'poolRewardsVault';
                        type: 'pubkey';
                    },
                    {
                        name: 'daoRewardsVault';
                        type: 'pubkey';
                    },
                    {
                        name: 'daoTotalRewardsAmount';
                        type: 'u128';
                    },
                    {
                        name: 'poolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 14];
                        };
                    },
                    {
                        name: 'reservePadding';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                ];
            };
        },
        {
            name: 'stakeOrUnStakeEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'userKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'tokenMint';
                        type: 'pubkey';
                    },
                    {
                        name: 'changeSupplyAmount';
                        type: 'u128';
                    },
                    {
                        name: 'userStake';
                        type: {
                            defined: {
                                name: 'userStake';
                            };
                        };
                    },
                ];
            };
        },
        {
            name: 'state';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'admin';
                        type: 'pubkey';
                    },
                    {
                        name: 'bumpSigner';
                        type: 'pubkey';
                    },
                    {
                        name: 'keeperKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'essentialAccountAlt';
                        type: 'pubkey';
                    },
                    {
                        name: 'fundingFeeBaseRate';
                        type: 'u128';
                    },
                    {
                        name: 'maximumFundingBaseRate';
                        type: 'u128';
                    },
                    {
                        name: 'minimumPrecisionMultiple';
                        type: 'u128';
                    },
                    {
                        name: 'poolRewardsIntervalLimit';
                        type: 'u128';
                    },
                    {
                        name: 'minimumOrderMarginUsd';
                        type: 'u128';
                    },
                    {
                        name: 'initFee';
                        type: 'u64';
                    },
                    {
                        name: 'tradingFeeUsdPoolRewardsRatio';
                        type: 'u32';
                    },
                    {
                        name: 'maximumMaintenanceMarginRate';
                        type: 'u32';
                    },
                    {
                        name: 'poolFeeRewardRatio';
                        type: 'u32';
                    },
                    {
                        name: 'marketSequence';
                        type: 'u16';
                    },
                    {
                        name: 'poolSequence';
                        type: 'u16';
                    },
                    {
                        name: 'tradeTokenSequence';
                        type: 'u16';
                    },
                    {
                        name: 'bumpSignerNonce';
                        type: 'u8';
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 5];
                        };
                    },
                    {
                        name: 'reservePadding';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                ];
            };
        },
        {
            name: 'stopType';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'none';
                    },
                    {
                        name: 'stopLoss';
                    },
                    {
                        name: 'takeProfit';
                    },
                ];
            };
        },
        {
            name: 'tradeToken';
            serialization: 'bytemuckunsafe';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'mintKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'totalLiability';
                        type: 'u128';
                    },
                    {
                        name: 'totalAmount';
                        type: 'u128';
                    },
                    {
                        name: 'oracleKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'vaultKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'name';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'discount';
                        type: 'u32';
                    },
                    {
                        name: 'liquidationFactor';
                        type: 'u32';
                    },
                    {
                        name: 'index';
                        type: 'u16';
                    },
                    {
                        name: 'decimals';
                        type: 'u16';
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 4];
                        };
                    },
                    {
                        name: 'reservePadding';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                ];
            };
        },
        {
            name: 'unStakeParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'share';
                        type: 'u128';
                    },
                    {
                        name: 'poolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'tradeTokenIndex';
                        type: 'u16';
                    },
                ];
            };
        },
        {
            name: 'updatePositionLeverageParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'symbol';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'isLong';
                        type: 'bool';
                    },
                    {
                        name: 'isPortfolioMargin';
                        type: 'bool';
                    },
                    {
                        name: 'leverage';
                        type: 'u32';
                    },
                    {
                        name: 'addMarginAmount';
                        type: 'u128';
                    },
                    {
                        name: 'marketIndex';
                        type: 'u16';
                    },
                    {
                        name: 'poolIndex';
                        type: 'u16';
                    },
                ];
            };
        },
        {
            name: 'updatePositionMarginParams';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'positionKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'isAdd';
                        type: 'bool';
                    },
                    {
                        name: 'updateMarginAmount';
                        type: 'u128';
                    },
                    {
                        name: 'addInitialMarginFromPortfolio';
                        type: 'u128';
                    },
                    {
                        name: 'marketIndex';
                        type: 'u16';
                    },
                    {
                        name: 'poolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'stablePoolIndex';
                        type: 'u16';
                    },
                    {
                        name: 'tradeTokenIndex';
                        type: 'u16';
                    },
                ];
            };
        },
        {
            name: 'updateUserPositionEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'prePosition';
                        type: {
                            defined: {
                                name: 'userPosition';
                            };
                        };
                    },
                    {
                        name: 'position';
                        type: {
                            defined: {
                                name: 'userPosition';
                            };
                        };
                    },
                ];
            };
        },
        {
            name: 'user';
            serialization: 'bytemuckunsafe';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'key';
                        type: 'pubkey';
                    },
                    {
                        name: 'nextOrderId';
                        type: 'u64';
                    },
                    {
                        name: 'nextLiquidationId';
                        type: 'u64';
                    },
                    {
                        name: 'hold';
                        type: 'u128';
                    },
                    {
                        name: 'tokens';
                        type: {
                            array: [
                                {
                                    defined: {
                                        name: 'userToken';
                                    };
                                },
                                10,
                            ];
                        };
                    },
                    {
                        name: 'stakes';
                        type: {
                            array: [
                                {
                                    defined: {
                                        name: 'userStake';
                                    };
                                },
                                10,
                            ];
                        };
                    },
                    {
                        name: 'positions';
                        type: {
                            array: [
                                {
                                    defined: {
                                        name: 'userPosition';
                                    };
                                },
                                10,
                            ];
                        };
                    },
                    {
                        name: 'orders';
                        type: {
                            array: [
                                {
                                    defined: {
                                        name: 'userOrder';
                                    };
                                },
                                8,
                            ];
                        };
                    },
                    {
                        name: 'authority';
                        type: 'pubkey';
                    },
                    {
                        name: 'createdAt';
                        type: 'i64';
                    },
                    {
                        name: 'userStatus';
                        type: {
                            defined: {
                                name: 'userStatus';
                            };
                        };
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 7];
                        };
                    },
                ];
            };
        },
        {
            name: 'userHoldUpdateEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'userKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'preHoldAmount';
                        type: 'u128';
                    },
                    {
                        name: 'holdAmount';
                        type: 'u128';
                    },
                ];
            };
        },
        {
            name: 'userOrder';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'orderMargin';
                        type: 'u128';
                    },
                    {
                        name: 'orderSize';
                        type: 'u128';
                    },
                    {
                        name: 'triggerPrice';
                        type: 'u128';
                    },
                    {
                        name: 'acceptablePrice';
                        type: 'u128';
                    },
                    {
                        name: 'createdAt';
                        type: 'i64';
                    },
                    {
                        name: 'orderId';
                        type: 'u64';
                    },
                    {
                        name: 'marginMintKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'authority';
                        type: 'pubkey';
                    },
                    {
                        name: 'userTokenAccount';
                        type: 'pubkey';
                    },
                    {
                        name: 'symbol';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'leverage';
                        type: 'u32';
                    },
                    {
                        name: 'orderSide';
                        type: {
                            defined: {
                                name: 'orderSide';
                            };
                        };
                    },
                    {
                        name: 'positionSide';
                        type: {
                            defined: {
                                name: 'positionSide';
                            };
                        };
                    },
                    {
                        name: 'orderType';
                        type: {
                            defined: {
                                name: 'orderType';
                            };
                        };
                    },
                    {
                        name: 'stopType';
                        type: {
                            defined: {
                                name: 'stopType';
                            };
                        };
                    },
                    {
                        name: 'status';
                        type: {
                            defined: {
                                name: 'orderStatus';
                            };
                        };
                    },
                    {
                        name: 'isPortfolioMargin';
                        type: 'bool';
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 6];
                        };
                    },
                    {
                        name: 'reservePadding';
                        type: {
                            array: ['u8', 16];
                        };
                    },
                ];
            };
        },
        {
            name: 'userPosition';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'positionSize';
                        type: 'u128';
                    },
                    {
                        name: 'entryPrice';
                        type: 'u128';
                    },
                    {
                        name: 'initialMargin';
                        type: 'u128';
                    },
                    {
                        name: 'initialMarginUsd';
                        type: 'u128';
                    },
                    {
                        name: 'initialMarginUsdFromPortfolio';
                        type: 'u128';
                    },
                    {
                        name: 'mmUsd';
                        type: 'u128';
                    },
                    {
                        name: 'holdPoolAmount';
                        type: 'u128';
                    },
                    {
                        name: 'openFee';
                        type: 'u128';
                    },
                    {
                        name: 'openFeeInUsd';
                        type: 'u128';
                    },
                    {
                        name: 'realizedBorrowingFee';
                        type: 'u128';
                    },
                    {
                        name: 'realizedBorrowingFeeInUsd';
                        type: 'u128';
                    },
                    {
                        name: 'openBorrowingFeePerToken';
                        type: 'u128';
                    },
                    {
                        name: 'realizedFundingFee';
                        type: 'i128';
                    },
                    {
                        name: 'realizedFundingFeeInUsd';
                        type: 'i128';
                    },
                    {
                        name: 'openFundingFeeAmountPerSize';
                        type: 'i128';
                    },
                    {
                        name: 'closeFeeInUsd';
                        type: 'u128';
                    },
                    {
                        name: 'realizedPnl';
                        type: 'i128';
                    },
                    {
                        name: 'userKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'userTokenAccount';
                        type: 'pubkey';
                    },
                    {
                        name: 'marginMintKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'indexMintOracle';
                        type: 'pubkey';
                    },
                    {
                        name: 'positionKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'symbol';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'updatedAt';
                        type: 'i64';
                    },
                    {
                        name: 'leverage';
                        type: 'u32';
                    },
                    {
                        name: 'isLong';
                        type: 'bool';
                    },
                    {
                        name: 'isPortfolioMargin';
                        type: 'bool';
                    },
                    {
                        name: 'status';
                        type: {
                            defined: {
                                name: 'positionStatus';
                            };
                        };
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 1];
                        };
                    },
                    {
                        name: 'reservePadding';
                        type: {
                            array: ['u8', 16];
                        };
                    },
                ];
            };
        },
        {
            name: 'userRewards';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'totalClaimRewardsAmount';
                        type: 'u128';
                    },
                    {
                        name: 'realisedRewardsTokenAmount';
                        type: 'u128';
                    },
                    {
                        name: 'openRewardsPerStakeToken';
                        type: 'u128';
                    },
                    {
                        name: 'tokenKey';
                        type: 'pubkey';
                    },
                ];
            };
        },
        {
            name: 'userRewardsUpdateEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'userKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'tokenMint';
                        type: 'pubkey';
                    },
                    {
                        name: 'userRewards';
                        type: {
                            defined: {
                                name: 'userRewards';
                            };
                        };
                    },
                ];
            };
        },
        {
            name: 'userStake';
            docs: [
                "Represents a user's staking information",
                '',
                "When a user stakes in an asset pool [`Pool`], a [`UserStake`] object is created to record the user's staking information.",
                'Like other user information, it is a reusable object. See: [`UserStakeStatus::INIT`]',
            ];
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'stakedShare';
                        docs: [
                            "User's staking shares",
                            '',
                            "This value represents the user's staking shares (in USD), calculated as:",
                            '((amount of staked tokens - fees) * current token value) / current net value of the pool',
                        ];
                        type: 'u128';
                    },
                    {
                        name: 'userRewards';
                        docs: ['Rewards earned by the user from staking'];
                        type: {
                            defined: {
                                name: 'userRewards';
                            };
                        };
                    },
                    {
                        name: 'poolKey';
                        docs: ['The pool in which the user has staked'];
                        type: 'pubkey';
                    },
                    {
                        name: 'userStakeStatus';
                        docs: ["The status of the user's stake"];
                        type: {
                            defined: {
                                name: 'userStakeStatus';
                            };
                        };
                    },
                    {
                        name: 'padding';
                        docs: ['Padding for alignment'];
                        type: {
                            array: ['u8', 15];
                        };
                    },
                    {
                        name: 'reservePadding';
                        docs: ['Reserved for future use'];
                        type: {
                            array: ['u8', 16];
                        };
                    },
                ];
            };
        },
        {
            name: 'userStakeStatus';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'init';
                    },
                    {
                        name: 'using';
                    },
                ];
            };
        },
        {
            name: 'userStatus';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'normal';
                    },
                    {
                        name: 'liquidation';
                    },
                    {
                        name: 'disable';
                    },
                ];
            };
        },
        {
            name: 'userToken';
            repr: {
                kind: 'c';
            };
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'amount';
                        type: 'u128';
                    },
                    {
                        name: 'usedAmount';
                        type: 'u128';
                    },
                    {
                        name: 'liabilityAmount';
                        type: 'u128';
                    },
                    {
                        name: 'tokenMintKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'userTokenStatus';
                        type: {
                            defined: {
                                name: 'userTokenStatus';
                            };
                        };
                    },
                    {
                        name: 'padding';
                        type: {
                            array: ['u8', 15];
                        };
                    },
                    {
                        name: 'reservePadding';
                        type: {
                            array: ['u8', 16];
                        };
                    },
                ];
            };
        },
        {
            name: 'userTokenBalanceUpdateEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'userKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'tokenMint';
                        type: 'pubkey';
                    },
                    {
                        name: 'preUserToken';
                        type: {
                            defined: {
                                name: 'userToken';
                            };
                        };
                    },
                    {
                        name: 'userToken';
                        type: {
                            defined: {
                                name: 'userToken';
                            };
                        };
                    },
                    {
                        name: 'updateOrigin';
                        type: {
                            defined: {
                                name: 'userTokenUpdateReason';
                            };
                        };
                    },
                ];
            };
        },
        {
            name: 'userTokenStatus';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'init';
                    },
                    {
                        name: 'using';
                    },
                ];
            };
        },
        {
            name: 'userTokenUpdateReason';
            type: {
                kind: 'enum';
                variants: [
                    {
                        name: 'default';
                    },
                    {
                        name: 'deposit';
                    },
                    {
                        name: 'withdraw';
                    },
                    {
                        name: 'settleFee';
                    },
                    {
                        name: 'settlePnl';
                    },
                    {
                        name: 'decreasePosition';
                    },
                    {
                        name: 'increasePosition';
                    },
                    {
                        name: 'updateLeverage';
                    },
                    {
                        name: 'collectOpenFee';
                    },
                    {
                        name: 'collectCloseFee';
                    },
                    {
                        name: 'transferToStake';
                    },
                    {
                        name: 'transferFromStake';
                    },
                    {
                        name: 'liquidateLiability';
                    },
                    {
                        name: 'liquidation';
                    },
                ];
            };
        },
        {
            name: 'withdrawEvent';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'userKey';
                        type: 'pubkey';
                    },
                    {
                        name: 'tokenMint';
                        type: 'pubkey';
                    },
                    {
                        name: 'amount';
                        type: 'u128';
                    },
                ];
            };
        },
    ];
};
