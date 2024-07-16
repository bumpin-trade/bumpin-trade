/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/bumpin_trade.json`.
 */
export type BumpinTrade = {
  "address": "AQkVcL5spcyrqiKNJykGWGD78ry8Erkuub2t2ogUVWca",
  "metadata": {
    "name": "bumpinTrade",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor"
  },
  "instructions": [
    {
      "name": "addPositionMargin",
      "discriminator": [
        52,
        123,
        95,
        117,
        1,
        134,
        241,
        181
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "tradeToken",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "params.trade_token_index"
              }
            ]
          }
        },
        {
          "name": "pool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "stablePool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "params.stable_pool_index"
              }
            ]
          }
        },
        {
          "name": "market",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  97,
                  114,
                  107,
                  101,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "params.market_index"
              }
            ]
          }
        },
        {
          "name": "poolMintVault",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "updatePositionMarginParams"
            }
          }
        }
      ]
    },
    {
      "name": "adl",
      "discriminator": [
        54,
        233,
        120,
        58,
        42,
        177,
        121,
        5
      ],
      "accounts": [
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "market",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  97,
                  114,
                  107,
                  101,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "marketIndex"
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "stablePool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "stablePoolIndex"
              }
            ]
          }
        },
        {
          "name": "poolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "stablePoolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "stablePoolIndex"
              }
            ]
          }
        },
        {
          "name": "tradeToken",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "tradeTokenIndex"
              }
            ]
          }
        },
        {
          "name": "tradeTokenVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "tradeTokenIndex"
              }
            ]
          }
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "poolIndex",
          "type": "u16"
        },
        {
          "name": "stablePoolIndex",
          "type": "u16"
        },
        {
          "name": "marketIndex",
          "type": "u16"
        },
        {
          "name": "tradeTokenIndex",
          "type": "u16"
        },
        {
          "name": "params",
          "type": {
            "array": [
              {
                "defined": {
                  "name": "adlParams"
                }
              },
              10
            ]
          }
        }
      ]
    },
    {
      "name": "autoCompound",
      "discriminator": [
        190,
        236,
        229,
        204,
        126,
        66,
        94,
        179
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "poolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "poolRewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "poolIndex",
          "type": "u16"
        }
      ]
    },
    {
      "name": "cancelOrder",
      "discriminator": [
        95,
        129,
        237,
        240,
        8,
        49,
        223,
        132
      ],
      "accounts": [
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "poolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "orderId",
          "type": "u64"
        },
        {
          "name": "poolIndex",
          "type": "u16"
        }
      ]
    },
    {
      "name": "claimRewards",
      "discriminator": [
        4,
        144,
        132,
        71,
        116,
        23,
        151,
        80
      ],
      "accounts": [
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "rewards",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "poolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "poolRewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "poolIndex",
          "type": "u16"
        }
      ]
    },
    {
      "name": "collectRewards",
      "discriminator": [
        63,
        130,
        90,
        197,
        39,
        16,
        143,
        176
      ],
      "accounts": [
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "poolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "stablePoolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "stablePoolIndex"
              }
            ]
          }
        },
        {
          "name": "tradeToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "tradeTokenIndex"
              }
            ]
          }
        },
        {
          "name": "stableTradeToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "stableTradeTokenIndex"
              }
            ]
          }
        },
        {
          "name": "rewards",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "poolRewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "daoRewardsVault",
          "writable": true
        },
        {
          "name": "keeperKey",
          "signer": true,
          "relations": [
            "state"
          ]
        },
        {
          "name": "bumpSigner",
          "relations": [
            "state"
          ]
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "poolIndex",
          "type": "u16"
        },
        {
          "name": "stablePoolIndex",
          "type": "u16"
        },
        {
          "name": "tradeTokenIndex",
          "type": "u16"
        },
        {
          "name": "stableTradeTokenIndex",
          "type": "u16"
        }
      ]
    },
    {
      "name": "deposit",
      "discriminator": [
        242,
        35,
        198,
        137,
        82,
        225,
        242,
        182
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "tradeToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "tokenIndex"
              }
            ]
          }
        },
        {
          "name": "tradeTokenVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "tokenIndex"
              }
            ]
          }
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "tokenIndex",
          "type": "u16"
        },
        {
          "name": "amount",
          "type": "u128"
        }
      ]
    },
    {
      "name": "executeOrder",
      "discriminator": [
        115,
        61,
        180,
        24,
        168,
        32,
        215,
        20
      ],
      "accounts": [
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "arg",
                "path": "userKey"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "keeperKey",
          "signer": true
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "orderId",
          "type": "u64"
        },
        {
          "name": "userKey",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "initializeMarket",
      "discriminator": [
        35,
        35,
        189,
        193,
        155,
        48,
        170,
        203
      ],
      "accounts": [
        {
          "name": "market",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  97,
                  114,
                  107,
                  101,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "state.market_sequence",
                "account": "state"
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "stablePool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "params.stable_pool_index"
              }
            ]
          }
        },
        {
          "name": "indexMintOracle"
        },
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "relations": [
            "state"
          ]
        },
        {
          "name": "state",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "initializeMarketParams"
            }
          }
        }
      ]
    },
    {
      "name": "initializePool",
      "discriminator": [
        95,
        180,
        10,
        172,
        84,
        174,
        232,
        40
      ],
      "accounts": [
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "account",
                "path": "state.pool_sequence",
                "account": "state"
              }
            ]
          }
        },
        {
          "name": "poolMint"
        },
        {
          "name": "poolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "account",
                "path": "state.pool_sequence",
                "account": "state"
              }
            ]
          }
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "state",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "relations": [
            "state"
          ]
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "initializePoolParams"
            }
          }
        }
      ]
    },
    {
      "name": "initializeRewards",
      "discriminator": [
        91,
        174,
        112,
        191,
        233,
        236,
        147,
        12
      ],
      "accounts": [
        {
          "name": "state",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "pool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "poolMint"
        },
        {
          "name": "rewards",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  114,
                  101,
                  119,
                  97,
                  114,
                  100,
                  115
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "poolRewardsVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "daoRewardsVault"
        },
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "relations": [
            "state"
          ]
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "poolIndex",
          "type": "u16"
        }
      ]
    },
    {
      "name": "initializeState",
      "discriminator": [
        190,
        171,
        224,
        219,
        217,
        72,
        199,
        176
      ],
      "accounts": [
        {
          "name": "admin",
          "writable": true,
          "signer": true
        },
        {
          "name": "state",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "param",
          "type": {
            "defined": {
              "name": "initializeStateParams"
            }
          }
        }
      ]
    },
    {
      "name": "initializeTradeToken",
      "discriminator": [
        188,
        220,
        217,
        110,
        223,
        180,
        96,
        121
      ],
      "accounts": [
        {
          "name": "tradeToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  110
                ]
              },
              {
                "kind": "account",
                "path": "state.trade_token_sequence",
                "account": "state"
              }
            ]
          }
        },
        {
          "name": "tradeTokenMint"
        },
        {
          "name": "tradeTokenVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "account",
                "path": "state.trade_token_sequence",
                "account": "state"
              }
            ]
          }
        },
        {
          "name": "oracle"
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "state",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "admin",
          "writable": true,
          "signer": true,
          "relations": [
            "state"
          ]
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "discount",
          "type": "u32"
        },
        {
          "name": "mintName",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
        },
        {
          "name": "liquidationFactor",
          "type": "u32"
        }
      ]
    },
    {
      "name": "initializeUser",
      "discriminator": [
        111,
        17,
        185,
        250,
        60,
        122,
        38,
        254
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },
    {
      "name": "liquidatePosition",
      "discriminator": [
        187,
        74,
        229,
        149,
        102,
        81,
        221,
        68
      ],
      "accounts": [
        {
          "name": "state",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "user",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "arg",
                "path": "userAuthorityKey"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "market",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  97,
                  114,
                  107,
                  101,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "marketIndex"
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "stablePool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "stablePoolIndex"
              }
            ]
          }
        },
        {
          "name": "poolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "stablePoolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "stablePoolIndex"
              }
            ]
          }
        },
        {
          "name": "tradeToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "tradeTokenIndex"
              }
            ]
          }
        },
        {
          "name": "tradeTokenVault",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "tradeTokenIndex"
              }
            ]
          }
        },
        {
          "name": "keeperSigner",
          "signer": true
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "positionKey",
          "type": "pubkey"
        },
        {
          "name": "liquidationPrice",
          "type": "u128"
        },
        {
          "name": "marketIndex",
          "type": "u16"
        },
        {
          "name": "poolIndex",
          "type": "u16"
        },
        {
          "name": "stablePoolIndex",
          "type": "u16"
        },
        {
          "name": "userAuthorityKey",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "placeOrder",
      "discriminator": [
        51,
        194,
        155,
        175,
        109,
        130,
        96,
        106
      ],
      "accounts": [
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "order",
          "type": {
            "defined": {
              "name": "placeOrderParams"
            }
          }
        }
      ]
    },
    {
      "name": "portfolioStake",
      "discriminator": [
        197,
        248,
        167,
        245,
        97,
        196,
        252,
        19
      ],
      "accounts": [
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "poolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "tradeTokenVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "tradeTokenIndex"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "poolIndex",
          "type": "u16"
        },
        {
          "name": "tradeTokenIndex",
          "type": "u16"
        },
        {
          "name": "requestTokenAmount",
          "type": "u128"
        }
      ]
    },
    {
      "name": "portfolioUnStake",
      "discriminator": [
        38,
        230,
        213,
        92,
        0,
        48,
        190,
        42
      ],
      "accounts": [
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "poolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "tradeTokenVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "params.trade_token_index"
              }
            ]
          }
        },
        {
          "name": "tradeToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "params.trade_token_index"
              }
            ]
          }
        },
        {
          "name": "poolRewardsVault",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "unStakeParams"
            }
          }
        }
      ]
    },
    {
      "name": "updatePositionLeverage",
      "discriminator": [
        193,
        183,
        118,
        54,
        175,
        135,
        124,
        132
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "pool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "stablePool",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "market",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  109,
                  97,
                  114,
                  107,
                  101,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "params.market_index"
              }
            ]
          }
        },
        {
          "name": "poolMintVault",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "updatePositionLeverageParams"
            }
          }
        }
      ]
    },
    {
      "name": "updateUserStatus",
      "discriminator": [
        4,
        129,
        231,
        220,
        216,
        44,
        151,
        55
      ],
      "accounts": [
        {
          "name": "state",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "user",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "arg",
                "path": "userAuthorityKey"
              }
            ]
          }
        },
        {
          "name": "keeperKey",
          "signer": true,
          "relations": [
            "state"
          ]
        }
      ],
      "args": [
        {
          "name": "userStatus",
          "type": {
            "defined": {
              "name": "userStatus"
            }
          }
        },
        {
          "name": "userAuthorityKey",
          "type": "pubkey"
        }
      ]
    },
    {
      "name": "walletStake",
      "discriminator": [
        133,
        206,
        14,
        95,
        182,
        13,
        19,
        134
      ],
      "accounts": [
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "poolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "poolIndex"
              }
            ]
          }
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "poolIndex",
          "type": "u16"
        },
        {
          "name": "tradeTokenIndex",
          "type": "u16"
        },
        {
          "name": "requestTokenAmount",
          "type": "u128"
        }
      ]
    },
    {
      "name": "walletUnStake",
      "discriminator": [
        233,
        75,
        110,
        151,
        143,
        26,
        47,
        229
      ],
      "accounts": [
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "user",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  117,
                  115,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "authority"
              }
            ]
          }
        },
        {
          "name": "pool",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "poolVault",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  111,
                  111,
                  108,
                  95,
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "tradeToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "params.trade_token_index"
              }
            ]
          }
        },
        {
          "name": "poolRewardsVault",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "params.pool_index"
              }
            ]
          }
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": {
              "name": "unStakeParams"
            }
          }
        }
      ]
    },
    {
      "name": "withdraw",
      "discriminator": [
        183,
        18,
        70,
        156,
        148,
        109,
        161,
        34
      ],
      "accounts": [
        {
          "name": "state",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  98,
                  117,
                  109,
                  112,
                  95,
                  115,
                  116,
                  97,
                  116,
                  101
                ]
              }
            ]
          }
        },
        {
          "name": "user",
          "writable": true
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "userTokenAccount",
          "writable": true
        },
        {
          "name": "tradeTokenVault",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  116
                ]
              },
              {
                "kind": "arg",
                "path": "tokenIndex"
              }
            ]
          }
        },
        {
          "name": "tradeToken",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
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
                  110
                ]
              },
              {
                "kind": "arg",
                "path": "tokenIndex"
              }
            ]
          }
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u128"
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "market",
      "discriminator": [
        219,
        190,
        213,
        55,
        0,
        227,
        198,
        154
      ]
    },
    {
      "name": "pool",
      "discriminator": [
        241,
        154,
        109,
        4,
        17,
        177,
        109,
        188
      ]
    },
    {
      "name": "rewards",
      "discriminator": [
        12,
        223,
        68,
        101,
        63,
        33,
        38,
        101
      ]
    },
    {
      "name": "state",
      "discriminator": [
        216,
        146,
        107,
        94,
        104,
        75,
        182,
        177
      ]
    },
    {
      "name": "tradeToken",
      "discriminator": [
        228,
        70,
        239,
        205,
        241,
        218,
        48,
        76
      ]
    },
    {
      "name": "user",
      "discriminator": [
        159,
        117,
        95,
        227,
        239,
        151,
        58,
        236
      ]
    }
  ],
  "events": [
    {
      "name": "addOrDecreaseMarginEvent",
      "discriminator": [
        255,
        34,
        9,
        229,
        17,
        100,
        24,
        85
      ]
    },
    {
      "name": "addOrDeleteUserOrderEvent",
      "discriminator": [
        226,
        197,
        6,
        41,
        154,
        69,
        25,
        217
      ]
    },
    {
      "name": "addOrDeleteUserPositionEvent",
      "discriminator": [
        62,
        52,
        185,
        169,
        23,
        84,
        198,
        48
      ]
    },
    {
      "name": "depositEvent",
      "discriminator": [
        120,
        248,
        61,
        83,
        31,
        142,
        107,
        144
      ]
    },
    {
      "name": "initUserEvent",
      "discriminator": [
        172,
        69,
        161,
        169,
        238,
        167,
        121,
        162
      ]
    },
    {
      "name": "poolUpdateEvent",
      "discriminator": [
        124,
        213,
        52,
        182,
        189,
        225,
        61,
        254
      ]
    },
    {
      "name": "stakeOrUnStakeEvent",
      "discriminator": [
        80,
        62,
        133,
        137,
        171,
        83,
        189,
        214
      ]
    },
    {
      "name": "updateUserPositionEvent",
      "discriminator": [
        102,
        98,
        41,
        238,
        50,
        237,
        214,
        100
      ]
    },
    {
      "name": "userHoldUpdateEvent",
      "discriminator": [
        134,
        142,
        93,
        85,
        54,
        61,
        131,
        93
      ]
    },
    {
      "name": "userRewardsUpdateEvent",
      "discriminator": [
        93,
        62,
        28,
        230,
        211,
        53,
        192,
        119
      ]
    },
    {
      "name": "userTokenBalanceUpdateEvent",
      "discriminator": [
        6,
        46,
        145,
        182,
        47,
        123,
        79,
        108
      ]
    },
    {
      "name": "withdrawEvent",
      "discriminator": [
        22,
        9,
        133,
        26,
        160,
        44,
        71,
        192
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "amountNotEnough",
      "msg": "amountNotEnough"
    },
    {
      "code": 6001,
      "name": "invalidTransfer",
      "msg": "Invalid transfer"
    },
    {
      "code": 6002,
      "name": "invalidParam",
      "msg": "invalidParam"
    },
    {
      "code": 6003,
      "name": "onlyOneShortOrderAllowed",
      "msg": "onlyOneShortOrderAllowed"
    },
    {
      "code": 6004,
      "name": "orderNotExist",
      "msg": "orderNotExist"
    },
    {
      "code": 6005,
      "name": "tokenNotMatch",
      "msg": "tokenNotMatch"
    },
    {
      "code": 6006,
      "name": "noMoreUserTokenSpace",
      "msg": "noMoreUserTokenSpace"
    },
    {
      "code": 6007,
      "name": "noMoreOrderSpace",
      "msg": "noMoreOrderSpace"
    },
    {
      "code": 6008,
      "name": "leverageIsNotAllowed",
      "msg": "leverageIsNotAllowed"
    },
    {
      "code": 6009,
      "name": "priceIsNotAllowed",
      "msg": "priceIsNotAllowed"
    },
    {
      "code": 6010,
      "name": "balanceNotEnough",
      "msg": "balanceNotEnough"
    },
    {
      "code": 6011,
      "name": "pythOffline",
      "msg": "pythOffline"
    },
    {
      "code": 6012,
      "name": "overflow",
      "msg": "overflow"
    },
    {
      "code": 6013,
      "name": "transferFailed",
      "msg": "transferFailed"
    },
    {
      "code": 6014,
      "name": "unableToLoadAccountLoader",
      "msg": "Unable to load AccountLoader"
    },
    {
      "code": 6015,
      "name": "cantPayUserInitFee",
      "msg": "cantPayUserInitFee"
    },
    {
      "code": 6016,
      "name": "couldNotFindUserToken",
      "msg": "couldNotFindUserToken"
    },
    {
      "code": 6017,
      "name": "couldNotFindUserOrder",
      "msg": "couldNotFindUserOrder"
    },
    {
      "code": 6018,
      "name": "couldNotFindUserPosition",
      "msg": "couldNotFindUserPosition"
    },
    {
      "code": 6019,
      "name": "onlyLiquidateIsolatePosition",
      "msg": "onlyLiquidateIsolatePosition"
    },
    {
      "code": 6020,
      "name": "onlyIsolatePositionAllowed",
      "msg": "onlyIsolatePositionAllowed"
    },
    {
      "code": 6021,
      "name": "couldNotFindUserStake",
      "msg": "couldNotFindUserStake"
    },
    {
      "code": 6022,
      "name": "oracleNotFound",
      "msg": "oracleNotFound"
    },
    {
      "code": 6023,
      "name": "oraclePriceToOld",
      "msg": "oraclePriceToOld"
    },
    {
      "code": 6024,
      "name": "unableToLoadOracle",
      "msg": "Unable To Load Oracles"
    },
    {
      "code": 6025,
      "name": "invalidOracle",
      "msg": "invalidOracle"
    },
    {
      "code": 6026,
      "name": "bnConversionError",
      "msg": "Conversion to u128/u128 failed with an overflow or underflow"
    },
    {
      "code": 6027,
      "name": "mathError",
      "msg": "Math Error"
    },
    {
      "code": 6028,
      "name": "castingFailure",
      "msg": "Casting Failure"
    },
    {
      "code": 6029,
      "name": "couldNotLoadMarketData",
      "msg": "couldNotLoadMarketData"
    },
    {
      "code": 6030,
      "name": "invalidMarketAccount",
      "msg": "invalidMarketAccount"
    },
    {
      "code": 6031,
      "name": "marketWrongMutability",
      "msg": "marketWrongMutability"
    },
    {
      "code": 6032,
      "name": "failedUnwrap",
      "msg": "Failed Unwrap"
    },
    {
      "code": 6033,
      "name": "userNotEnoughValue",
      "msg": "User Not Enough Value"
    },
    {
      "code": 6034,
      "name": "amountZero",
      "msg": "amountZero"
    },
    {
      "code": 6035,
      "name": "couldNotLoadTokenAccountData",
      "msg": "couldNotLoadTokenAccountData"
    },
    {
      "code": 6036,
      "name": "couldNotLoadTradeTokenData",
      "msg": "couldNotLoadTradeTokenData"
    },
    {
      "code": 6037,
      "name": "couldNotLoadPoolData",
      "msg": "couldNotLoadPoolData"
    },
    {
      "code": 6038,
      "name": "invalidTradeTokenAccount",
      "msg": "invalidTradeTokenAccount"
    },
    {
      "code": 6039,
      "name": "invalidTokenAccount",
      "msg": "invalidTokenAccount"
    },
    {
      "code": 6040,
      "name": "invalidPoolAccount",
      "msg": "invalidPoolAccount"
    },
    {
      "code": 6041,
      "name": "tradeTokenNotFind",
      "msg": "canNotFindTradeToken"
    },
    {
      "code": 6042,
      "name": "marketNotFind",
      "msg": "canNotFindMarket"
    },
    {
      "code": 6043,
      "name": "stakePaused",
      "msg": "stakePaused"
    },
    {
      "code": 6044,
      "name": "stakeToSmall",
      "msg": "stakeToSmall"
    },
    {
      "code": 6045,
      "name": "unStakeTooSmall",
      "msg": "unStakeTooSmall"
    },
    {
      "code": 6046,
      "name": "unStakeTooLarge",
      "msg": "unStakeTooLarge"
    },
    {
      "code": 6047,
      "name": "positionSideNotSupport",
      "msg": "positionSideNotSupport"
    },
    {
      "code": 6048,
      "name": "rewardsNotFound",
      "msg": "rewardsNotFound"
    },
    {
      "code": 6049,
      "name": "userNotFound",
      "msg": "userNotFound"
    },
    {
      "code": 6050,
      "name": "couldNotLoadUserData",
      "msg": "couldNotLoadUserData"
    },
    {
      "code": 6051,
      "name": "poolSubUnsettleNotEnough",
      "msg": "poolSubUnsettleNotEnough"
    },
    {
      "code": 6052,
      "name": "timestampNotFound",
      "msg": "timestampNotFound"
    },
    {
      "code": 6053,
      "name": "claimUnqualified",
      "msg": "claimUnqualified"
    },
    {
      "code": 6054,
      "name": "poolMintSupplyIsZero",
      "msg": "poolMintSupplyIsZero"
    }
  ],
  "types": [
    {
      "name": "adlParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "positionKey",
            "type": "pubkey"
          },
          {
            "name": "userKey",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "addOrDecreaseMarginEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "userKey",
            "type": "pubkey"
          },
          {
            "name": "position",
            "type": {
              "defined": {
                "name": "userPosition"
              }
            }
          },
          {
            "name": "prePosition",
            "type": {
              "defined": {
                "name": "userPosition"
              }
            }
          },
          {
            "name": "isAdd",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "addOrDeleteUserOrderEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "userKey",
            "type": "pubkey"
          },
          {
            "name": "order",
            "type": {
              "defined": {
                "name": "userOrder"
              }
            }
          },
          {
            "name": "isAdd",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "addOrDeleteUserPositionEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "position",
            "type": {
              "defined": {
                "name": "userPosition"
              }
            }
          },
          {
            "name": "isAdd",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "borrowingFee",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalBorrowingFee",
            "type": "u128"
          },
          {
            "name": "totalRealizedBorrowingFee",
            "type": "u128"
          },
          {
            "name": "cumulativeBorrowingFeePerToken",
            "type": "u128"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    },
    {
      "name": "depositEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "userKey",
            "type": "pubkey"
          },
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u128"
          },
          {
            "name": "depositOrigin",
            "type": {
              "defined": {
                "name": "depositOrigin"
              }
            }
          }
        ]
      }
    },
    {
      "name": "depositOrigin",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "manual"
          },
          {
            "name": "order"
          },
          {
            "name": "stake"
          }
        ]
      }
    },
    {
      "name": "feeReward",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "feeAmount",
            "type": "u128"
          },
          {
            "name": "unSettleFeeAmount",
            "type": "u128"
          },
          {
            "name": "cumulativeRewardsPerStakeToken",
            "type": "u128"
          },
          {
            "name": "lastRewardsPerStakeTokenDeltas",
            "type": {
              "array": [
                "u128",
                3
              ]
            }
          }
        ]
      }
    },
    {
      "name": "initUserEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "userKey",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "initializeMarketParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "symbol",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "tickSize",
            "type": "u128"
          },
          {
            "name": "openFeeRate",
            "type": "u128"
          },
          {
            "name": "closeFeeRate",
            "type": "u128"
          },
          {
            "name": "maximumLongOpenInterestCap",
            "type": "u128"
          },
          {
            "name": "maximumShortOpenInterestCap",
            "type": "u128"
          },
          {
            "name": "longShortRatioLimit",
            "type": "u128"
          },
          {
            "name": "longShortOiBottomLimit",
            "type": "u128"
          },
          {
            "name": "maximumLeverage",
            "type": "u32"
          },
          {
            "name": "minimumLeverage",
            "type": "u32"
          },
          {
            "name": "poolIndex",
            "type": "u16"
          },
          {
            "name": "stablePoolIndex",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "initializePoolParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "stableMintKey",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "poolConfig",
            "type": {
              "defined": {
                "name": "poolConfig"
              }
            }
          },
          {
            "name": "stable",
            "type": "bool"
          }
        ]
      }
    },
    {
      "name": "initializeStateParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "keeperKey",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "minOrderMarginUsd",
            "type": "u128"
          },
          {
            "name": "maximumMaintenanceMarginRate",
            "type": "u32"
          },
          {
            "name": "fundingFeeBaseRate",
            "type": "u128"
          },
          {
            "name": "maxFundingBaseRate",
            "type": "u128"
          },
          {
            "name": "tradingFeeStakingRewardsRatio",
            "type": "u128"
          },
          {
            "name": "tradingFeePoolRewardsRatio",
            "type": "u128"
          },
          {
            "name": "tradingFeeUsdPoolRewardsRatio",
            "type": "u128"
          },
          {
            "name": "borrowingFeeStakingRewardsRatio",
            "type": "u128"
          },
          {
            "name": "borrowingFeePoolRewardsRatio",
            "type": "u128"
          },
          {
            "name": "minPrecisionMultiple",
            "type": "u128"
          },
          {
            "name": "mintFeeStakingRewardsRatio",
            "type": "u128"
          },
          {
            "name": "mintFeePoolRewardsRatio",
            "type": "u128"
          },
          {
            "name": "redeemFeeStakingRewardsRatio",
            "type": "u128"
          },
          {
            "name": "redeemFeePoolRewardsRatio",
            "type": "u128"
          },
          {
            "name": "poolRewardsIntervalLimit",
            "type": "u128"
          },
          {
            "name": "initFee",
            "type": "u64"
          },
          {
            "name": "stakingFeeRewardRatio",
            "type": "u32"
          },
          {
            "name": "poolFeeRewardRatio",
            "type": "u32"
          }
        ]
      }
    },
    {
      "name": "market",
      "serialization": "bytemuckunsafe",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "symbol",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "longOpenInterest",
            "type": {
              "defined": {
                "name": "marketPosition"
              }
            }
          },
          {
            "name": "shortOpenInterest",
            "type": {
              "defined": {
                "name": "marketPosition"
              }
            }
          },
          {
            "name": "fundingFee",
            "type": {
              "defined": {
                "name": "marketFundingFee"
              }
            }
          },
          {
            "name": "config",
            "type": {
              "defined": {
                "name": "marketConfig"
              }
            }
          },
          {
            "name": "poolKey",
            "type": "pubkey"
          },
          {
            "name": "poolMintKey",
            "type": "pubkey"
          },
          {
            "name": "indexMintOracle",
            "type": "pubkey"
          },
          {
            "name": "stablePoolKey",
            "type": "pubkey"
          },
          {
            "name": "stablePoolMintKey",
            "type": "pubkey"
          },
          {
            "name": "index",
            "type": "u16"
          },
          {
            "name": "marketStatus",
            "type": {
              "defined": {
                "name": "marketStatus"
              }
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                13
              ]
            }
          },
          {
            "name": "reservePadding",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "marketConfig",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tickSize",
            "type": "u128"
          },
          {
            "name": "openFeeRate",
            "type": "u128"
          },
          {
            "name": "closeFeeRate",
            "type": "u128"
          },
          {
            "name": "maximumLongOpenInterestCap",
            "type": "u128"
          },
          {
            "name": "maximumShortOpenInterestCap",
            "type": "u128"
          },
          {
            "name": "longShortRatioLimit",
            "type": "u128"
          },
          {
            "name": "longShortOiBottomLimit",
            "type": "u128"
          },
          {
            "name": "maximumLeverage",
            "type": "u32"
          },
          {
            "name": "minimumLeverage",
            "type": "u32"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    },
    {
      "name": "marketFundingFee",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "longFundingFeeAmountPerSize",
            "type": "i128"
          },
          {
            "name": "shortFundingFeeAmountPerSize",
            "type": "i128"
          },
          {
            "name": "totalLongFundingFee",
            "type": "i128"
          },
          {
            "name": "totalShortFundingFee",
            "type": "i128"
          },
          {
            "name": "longFundingFeeRate",
            "type": "i128"
          },
          {
            "name": "shortFundingFeeRate",
            "type": "i128"
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          }
        ]
      }
    },
    {
      "name": "marketPosition",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "openInterest",
            "type": "u128"
          },
          {
            "name": "entryPrice",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "marketStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "normal"
          },
          {
            "name": "reduceOnly"
          },
          {
            "name": "pause"
          }
        ]
      }
    },
    {
      "name": "orderSide",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "long"
          },
          {
            "name": "short"
          }
        ]
      }
    },
    {
      "name": "orderStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "init"
          },
          {
            "name": "using"
          }
        ]
      }
    },
    {
      "name": "orderType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "market"
          },
          {
            "name": "limit"
          },
          {
            "name": "stop"
          }
        ]
      }
    },
    {
      "name": "placeOrderParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "symbol",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "size",
            "type": "u128"
          },
          {
            "name": "orderMargin",
            "type": "u128"
          },
          {
            "name": "leverage",
            "type": "u32"
          },
          {
            "name": "triggerPrice",
            "type": "u128"
          },
          {
            "name": "acceptablePrice",
            "type": "u128"
          },
          {
            "name": "placeTime",
            "type": "i64"
          },
          {
            "name": "poolIndex",
            "type": "u16"
          },
          {
            "name": "stablePoolIndex",
            "type": "u16"
          },
          {
            "name": "marketIndex",
            "type": "u16"
          },
          {
            "name": "tradeTokenIndex",
            "type": "u16"
          },
          {
            "name": "stableTradeTokenIndex",
            "type": "u16"
          },
          {
            "name": "isPortfolioMargin",
            "type": "bool"
          },
          {
            "name": "isNativeToken",
            "type": "bool"
          },
          {
            "name": "orderSide",
            "type": {
              "defined": {
                "name": "orderSide"
              }
            }
          },
          {
            "name": "positionSide",
            "type": {
              "defined": {
                "name": "positionSide"
              }
            }
          },
          {
            "name": "orderType",
            "type": {
              "defined": {
                "name": "orderType"
              }
            }
          },
          {
            "name": "stopType",
            "type": {
              "defined": {
                "name": "stopType"
              }
            }
          },
          {
            "name": "orderId",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "pool",
      "serialization": "bytemuckunsafe",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "type": "pubkey"
          },
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "pnl",
            "type": "i128"
          },
          {
            "name": "apr",
            "type": "u128"
          },
          {
            "name": "insuranceFundAmount",
            "type": "u128"
          },
          {
            "name": "totalSupply",
            "type": "u128"
          },
          {
            "name": "balance",
            "type": {
              "defined": {
                "name": "poolBalance"
              }
            }
          },
          {
            "name": "stableBalance",
            "type": {
              "defined": {
                "name": "poolBalance"
              }
            }
          },
          {
            "name": "borrowingFee",
            "type": {
              "defined": {
                "name": "borrowingFee"
              }
            }
          },
          {
            "name": "feeReward",
            "type": {
              "defined": {
                "name": "feeReward"
              }
            }
          },
          {
            "name": "stableFeeReward",
            "type": {
              "defined": {
                "name": "feeReward"
              }
            }
          },
          {
            "name": "config",
            "type": {
              "defined": {
                "name": "poolConfig"
              }
            }
          },
          {
            "name": "poolVaultKey",
            "type": "pubkey"
          },
          {
            "name": "stableMintKey",
            "type": "pubkey"
          },
          {
            "name": "mintKey",
            "type": "pubkey"
          },
          {
            "name": "index",
            "type": "u16"
          },
          {
            "name": "status",
            "type": {
              "defined": {
                "name": "poolStatus"
              }
            }
          },
          {
            "name": "stable",
            "type": "bool"
          },
          {
            "name": "marketNumber",
            "type": "u16"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                8
              ]
            }
          },
          {
            "name": "reservePadding",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "poolBalance",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "settleFundingFee",
            "type": "i128"
          },
          {
            "name": "amount",
            "type": "u128"
          },
          {
            "name": "holdAmount",
            "type": "u128"
          },
          {
            "name": "unSettleAmount",
            "type": "u128"
          },
          {
            "name": "settleFundingFeeAmount",
            "type": "u128"
          },
          {
            "name": "lossAmount",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "poolConfig",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "minimumStakeAmount",
            "type": "u128"
          },
          {
            "name": "minimumUnStakeAmount",
            "type": "u128"
          },
          {
            "name": "poolLiquidityLimit",
            "type": "u128"
          },
          {
            "name": "borrowingInterestRate",
            "type": "u128"
          },
          {
            "name": "stakeFeeRate",
            "type": "u32"
          },
          {
            "name": "unStakeFeeRate",
            "type": "u32"
          },
          {
            "name": "unSettleMintRatioLimit",
            "type": "u32"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          }
        ]
      }
    },
    {
      "name": "poolStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "normal"
          },
          {
            "name": "stakePaused"
          },
          {
            "name": "unStakePaused"
          }
        ]
      }
    },
    {
      "name": "poolUpdateEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poolKey",
            "type": "pubkey"
          },
          {
            "name": "poolMint",
            "type": "pubkey"
          },
          {
            "name": "poolIndex",
            "type": "u16"
          },
          {
            "name": "poolBalance",
            "type": {
              "defined": {
                "name": "poolBalance"
              }
            }
          },
          {
            "name": "stableBalance",
            "type": {
              "defined": {
                "name": "poolBalance"
              }
            }
          },
          {
            "name": "borrowingFee",
            "type": {
              "defined": {
                "name": "borrowingFee"
              }
            }
          },
          {
            "name": "feeReward",
            "type": {
              "defined": {
                "name": "feeReward"
              }
            }
          },
          {
            "name": "stableFeeReward",
            "type": {
              "defined": {
                "name": "feeReward"
              }
            }
          },
          {
            "name": "totalSupply",
            "type": "u128"
          },
          {
            "name": "pnl",
            "type": "i128"
          },
          {
            "name": "apr",
            "type": "u128"
          },
          {
            "name": "insuranceFundAmount",
            "type": "u128"
          },
          {
            "name": "prePoolBalance",
            "type": {
              "defined": {
                "name": "poolBalance"
              }
            }
          },
          {
            "name": "preStableBalance",
            "type": {
              "defined": {
                "name": "poolBalance"
              }
            }
          },
          {
            "name": "preBorrowingFee",
            "type": {
              "defined": {
                "name": "borrowingFee"
              }
            }
          },
          {
            "name": "preFeeReward",
            "type": {
              "defined": {
                "name": "feeReward"
              }
            }
          },
          {
            "name": "preStableFeeReward",
            "type": {
              "defined": {
                "name": "feeReward"
              }
            }
          },
          {
            "name": "preTotalSupply",
            "type": "u128"
          },
          {
            "name": "prePnl",
            "type": "i128"
          },
          {
            "name": "preApr",
            "type": "u128"
          },
          {
            "name": "preInsuranceFundAmount",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "positionSide",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "increase"
          },
          {
            "name": "decrease"
          }
        ]
      }
    },
    {
      "name": "positionStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "init"
          },
          {
            "name": "using"
          }
        ]
      }
    },
    {
      "name": "rewards",
      "serialization": "bytemuckunsafe",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "poolUnClaimAmount",
            "type": "u128"
          },
          {
            "name": "poolTotalRewardsAmount",
            "type": "u128"
          },
          {
            "name": "poolRewardsVault",
            "type": "pubkey"
          },
          {
            "name": "daoRewardsVault",
            "type": "pubkey"
          },
          {
            "name": "daoTotalRewardsAmount",
            "type": "u128"
          },
          {
            "name": "poolIndex",
            "type": "u16"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                14
              ]
            }
          },
          {
            "name": "reservePadding",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "stakeOrUnStakeEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "userKey",
            "type": "pubkey"
          },
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "changeSupplyAmount",
            "type": "u128"
          },
          {
            "name": "userStake",
            "type": {
              "defined": {
                "name": "userStake"
              }
            }
          }
        ]
      }
    },
    {
      "name": "state",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "admin",
            "type": "pubkey"
          },
          {
            "name": "bumpSigner",
            "type": "pubkey"
          },
          {
            "name": "keeperKey",
            "type": "pubkey"
          },
          {
            "name": "bumpSignerNonce",
            "type": "u8"
          },
          {
            "name": "marketSequence",
            "type": "u16"
          },
          {
            "name": "poolSequence",
            "type": "u16"
          },
          {
            "name": "tradeTokenSequence",
            "type": "u16"
          },
          {
            "name": "minimumOrderMarginUsd",
            "type": "u128"
          },
          {
            "name": "maximumMaintenanceMarginRate",
            "type": "u32"
          },
          {
            "name": "fundingFeeBaseRate",
            "type": "u128"
          },
          {
            "name": "maximumFundingBaseRate",
            "type": "u128"
          },
          {
            "name": "minimumPrecisionMultiple",
            "type": "u128"
          },
          {
            "name": "poolRewardsIntervalLimit",
            "type": "u128"
          },
          {
            "name": "initFee",
            "type": "u64"
          },
          {
            "name": "tradingFeeUsdPoolRewardsRatio",
            "type": "u32"
          },
          {
            "name": "poolFeeRewardRatio",
            "type": "u32"
          },
          {
            "name": "reservePadding",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "stopType",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "none"
          },
          {
            "name": "stopLoss"
          },
          {
            "name": "takeProfit"
          }
        ]
      }
    },
    {
      "name": "tradeToken",
      "serialization": "bytemuckunsafe",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "mintKey",
            "type": "pubkey"
          },
          {
            "name": "totalLiability",
            "type": "u128"
          },
          {
            "name": "totalAmount",
            "type": "u128"
          },
          {
            "name": "oracleKey",
            "type": "pubkey"
          },
          {
            "name": "vaultKey",
            "type": "pubkey"
          },
          {
            "name": "name",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "discount",
            "type": "u32"
          },
          {
            "name": "liquidationFactor",
            "type": "u32"
          },
          {
            "name": "index",
            "type": "u16"
          },
          {
            "name": "decimals",
            "type": "u16"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                4
              ]
            }
          },
          {
            "name": "reservePadding",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          }
        ]
      }
    },
    {
      "name": "unStakeParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "share",
            "type": "u128"
          },
          {
            "name": "poolIndex",
            "type": "u16"
          },
          {
            "name": "tradeTokenIndex",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "updatePositionLeverageParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "symbol",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "isLong",
            "type": "bool"
          },
          {
            "name": "isPortfolioMargin",
            "type": "bool"
          },
          {
            "name": "leverage",
            "type": "u32"
          },
          {
            "name": "addMarginAmount",
            "type": "u128"
          },
          {
            "name": "marketIndex",
            "type": "u16"
          },
          {
            "name": "poolIndex",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "updatePositionMarginParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "positionKey",
            "type": "pubkey"
          },
          {
            "name": "isAdd",
            "type": "bool"
          },
          {
            "name": "updateMarginAmount",
            "type": "u128"
          },
          {
            "name": "addInitialMarginFromPortfolio",
            "type": "u128"
          },
          {
            "name": "marketIndex",
            "type": "u16"
          },
          {
            "name": "poolIndex",
            "type": "u16"
          },
          {
            "name": "stablePoolIndex",
            "type": "u16"
          },
          {
            "name": "tradeTokenIndex",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "updateUserPositionEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "prePosition",
            "type": {
              "defined": {
                "name": "userPosition"
              }
            }
          },
          {
            "name": "position",
            "type": {
              "defined": {
                "name": "userPosition"
              }
            }
          }
        ]
      }
    },
    {
      "name": "user",
      "serialization": "bytemuckunsafe",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "type": "pubkey"
          },
          {
            "name": "nextOrderId",
            "type": "u64"
          },
          {
            "name": "nextLiquidationId",
            "type": "u64"
          },
          {
            "name": "hold",
            "type": "u128"
          },
          {
            "name": "tokens",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "userToken"
                  }
                },
                10
              ]
            }
          },
          {
            "name": "stakes",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "userStake"
                  }
                },
                10
              ]
            }
          },
          {
            "name": "positions",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "userPosition"
                  }
                },
                10
              ]
            }
          },
          {
            "name": "orders",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "userOrder"
                  }
                },
                10
              ]
            }
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "userStatus",
            "type": {
              "defined": {
                "name": "userStatus"
              }
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                15
              ]
            }
          },
          {
            "name": "reservePadding",
            "type": {
              "array": [
                "u8",
                288
              ]
            }
          }
        ]
      }
    },
    {
      "name": "userHoldUpdateEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "userKey",
            "type": "pubkey"
          },
          {
            "name": "preHoldAmount",
            "type": "u128"
          },
          {
            "name": "holdAmount",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "userOrder",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "orderMargin",
            "type": "u128"
          },
          {
            "name": "orderSize",
            "type": "u128"
          },
          {
            "name": "triggerPrice",
            "type": "u128"
          },
          {
            "name": "acceptablePrice",
            "type": "u128"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "orderId",
            "type": "u64"
          },
          {
            "name": "marginMintKey",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "symbol",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "leverage",
            "type": "u32"
          },
          {
            "name": "orderSide",
            "type": {
              "defined": {
                "name": "orderSide"
              }
            }
          },
          {
            "name": "positionSide",
            "type": {
              "defined": {
                "name": "positionSide"
              }
            }
          },
          {
            "name": "orderType",
            "type": {
              "defined": {
                "name": "orderType"
              }
            }
          },
          {
            "name": "stopType",
            "type": {
              "defined": {
                "name": "stopType"
              }
            }
          },
          {
            "name": "status",
            "type": {
              "defined": {
                "name": "orderStatus"
              }
            }
          },
          {
            "name": "isPortfolioMargin",
            "type": "bool"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          },
          {
            "name": "reservePadding",
            "type": {
              "array": [
                "u8",
                16
              ]
            }
          }
        ]
      }
    },
    {
      "name": "userPosition",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "positionSize",
            "type": "u128"
          },
          {
            "name": "entryPrice",
            "type": "u128"
          },
          {
            "name": "initialMargin",
            "type": "u128"
          },
          {
            "name": "initialMarginUsd",
            "type": "u128"
          },
          {
            "name": "initialMarginUsdFromPortfolio",
            "type": "u128"
          },
          {
            "name": "mmUsd",
            "type": "u128"
          },
          {
            "name": "holdPoolAmount",
            "type": "u128"
          },
          {
            "name": "openFee",
            "type": "u128"
          },
          {
            "name": "openFeeInUsd",
            "type": "u128"
          },
          {
            "name": "realizedBorrowingFee",
            "type": "u128"
          },
          {
            "name": "realizedBorrowingFeeInUsd",
            "type": "u128"
          },
          {
            "name": "openBorrowingFeePerToken",
            "type": "u128"
          },
          {
            "name": "realizedFundingFee",
            "type": "i128"
          },
          {
            "name": "realizedFundingFeeInUsd",
            "type": "i128"
          },
          {
            "name": "openFundingFeeAmountPerSize",
            "type": "i128"
          },
          {
            "name": "closeFeeInUsd",
            "type": "u128"
          },
          {
            "name": "realizedPnl",
            "type": "i128"
          },
          {
            "name": "userKey",
            "type": "pubkey"
          },
          {
            "name": "marginMintKey",
            "type": "pubkey"
          },
          {
            "name": "indexMintOracle",
            "type": "pubkey"
          },
          {
            "name": "positionKey",
            "type": "pubkey"
          },
          {
            "name": "symbol",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
          },
          {
            "name": "updatedAt",
            "type": "i64"
          },
          {
            "name": "leverage",
            "type": "u32"
          },
          {
            "name": "isLong",
            "type": "bool"
          },
          {
            "name": "isPortfolioMargin",
            "type": "bool"
          },
          {
            "name": "status",
            "type": {
              "defined": {
                "name": "positionStatus"
              }
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                1
              ]
            }
          },
          {
            "name": "reservePadding",
            "type": {
              "array": [
                "u8",
                16
              ]
            }
          }
        ]
      }
    },
    {
      "name": "userRewards",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalClaimRewardsAmount",
            "type": "u128"
          },
          {
            "name": "realisedRewardsTokenAmount",
            "type": "u128"
          },
          {
            "name": "openRewardsPerStakeToken",
            "type": "u128"
          },
          {
            "name": "tokenKey",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "userRewardsUpdateEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "userKey",
            "type": "pubkey"
          },
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "userRewards",
            "type": {
              "defined": {
                "name": "userRewards"
              }
            }
          }
        ]
      }
    },
    {
      "name": "userStake",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakedShare",
            "type": "u128"
          },
          {
            "name": "userRewards",
            "type": {
              "defined": {
                "name": "userRewards"
              }
            }
          },
          {
            "name": "poolKey",
            "type": "pubkey"
          },
          {
            "name": "userStakeStatus",
            "type": {
              "defined": {
                "name": "userStakeStatus"
              }
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                15
              ]
            }
          },
          {
            "name": "reservePadding",
            "type": {
              "array": [
                "u8",
                16
              ]
            }
          }
        ]
      }
    },
    {
      "name": "userStakeStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "init"
          },
          {
            "name": "using"
          }
        ]
      }
    },
    {
      "name": "userStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "normal"
          },
          {
            "name": "liquidation"
          },
          {
            "name": "disable"
          }
        ]
      }
    },
    {
      "name": "userToken",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "amount",
            "type": "u128"
          },
          {
            "name": "usedAmount",
            "type": "u128"
          },
          {
            "name": "liabilityAmount",
            "type": "u128"
          },
          {
            "name": "tokenMintKey",
            "type": "pubkey"
          },
          {
            "name": "userTokenAccountKey",
            "type": "pubkey"
          },
          {
            "name": "userTokenStatus",
            "type": {
              "defined": {
                "name": "userTokenStatus"
              }
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                15
              ]
            }
          },
          {
            "name": "reservePadding",
            "type": {
              "array": [
                "u8",
                16
              ]
            }
          }
        ]
      }
    },
    {
      "name": "userTokenBalanceUpdateEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "userKey",
            "type": "pubkey"
          },
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "preUserToken",
            "type": {
              "defined": {
                "name": "userToken"
              }
            }
          },
          {
            "name": "userToken",
            "type": {
              "defined": {
                "name": "userToken"
              }
            }
          },
          {
            "name": "updateOrigin",
            "type": {
              "defined": {
                "name": "userTokenUpdateReason"
              }
            }
          }
        ]
      }
    },
    {
      "name": "userTokenStatus",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "init"
          },
          {
            "name": "using"
          }
        ]
      }
    },
    {
      "name": "userTokenUpdateReason",
      "type": {
        "kind": "enum",
        "variants": [
          {
            "name": "default"
          },
          {
            "name": "deposit"
          },
          {
            "name": "withdraw"
          },
          {
            "name": "settleFee"
          },
          {
            "name": "settlePnl"
          },
          {
            "name": "decreasePosition"
          },
          {
            "name": "increasePosition"
          },
          {
            "name": "updateLeverage"
          },
          {
            "name": "collectOpenFee"
          },
          {
            "name": "collectCloseFee"
          },
          {
            "name": "transferToStake"
          },
          {
            "name": "transferFromStake"
          },
          {
            "name": "liquidateLiability"
          },
          {
            "name": "liquidation"
          }
        ]
      }
    },
    {
      "name": "withdrawEvent",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "userKey",
            "type": "pubkey"
          },
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u128"
          }
        ]
      }
    }
  ]
};
