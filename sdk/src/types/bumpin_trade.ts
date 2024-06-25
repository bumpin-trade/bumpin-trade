/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/bumpin_trade.json`.
 */
export type BumpinTrade = {
  "address": "3vxLXJqLqF3JG5TCbYycbKWRBbCJQLxQmBGCkyqEEefL",
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
                "path": "tradeTokenIndex"
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
                "path": "marketIndex"
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
          "name": "params",
          "type": {
            "defined": {
              "name": "updatePositionMarginParams"
            }
          }
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
          "name": "tradeTokenIndex",
          "type": "u16"
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
          "name": "pool",
          "writable": true
        },
        {
          "name": "stablePool",
          "writable": true
        },
        {
          "name": "market"
        },
        {
          "name": "state"
        },
        {
          "name": "poolVault",
          "writable": true
        },
        {
          "name": "stablePoolVault",
          "writable": true
        },
        {
          "name": "tradeToken"
        },
        {
          "name": "tradeTokenVault"
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
        }
      ],
      "args": [
        {
          "name": "stableTradeTokenIndex",
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
          "writable": true
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
          "type": "u128"
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
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
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
          "writable": true
        },
        {
          "name": "authority",
          "signer": true
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
          "name": "marginToken"
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
          "name": "indexTradeToken",
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
                "path": "indexTradeTokenIndex"
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
          "type": "u128"
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
          "name": "indexTradeTokenIndex",
          "type": "u16"
        }
      ]
    },
    {
      "name": "initialize",
      "discriminator": [
        175,
        175,
        109,
        31,
        13,
        152,
        155,
        237
      ],
      "accounts": [
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
              }
            ]
          }
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tradeTokenMint"
        },
        {
          "name": "admin",
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
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
    },
    {
      "name": "initialize1",
      "discriminator": [
        217,
        221,
        202,
        48,
        229,
        106,
        212,
        42
      ],
      "accounts": [
        {
          "name": "keyValue",
          "writable": true,
          "signer": true
        },
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
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
                "path": "state.number_of_markets",
                "account": "state"
              }
            ]
          }
        },
        {
          "name": "pool"
        },
        {
          "name": "stablePool"
        },
        {
          "name": "indexMint"
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
          "name": "symbol",
          "type": {
            "array": [
              "u8",
              32
            ]
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
                "path": "state.number_of_pools",
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
                "path": "state.number_of_pools",
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
          "name": "name",
          "type": {
            "array": [
              "u8",
              32
            ]
          }
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
                "path": "state.number_of_trade_tokens",
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
                "path": "state.number_of_trade_tokens",
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
          "type": "u128"
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
          "type": "u128"
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
          "name": "state"
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
      "name": "liquidateCrossPosition",
      "discriminator": [
        40,
        173,
        153,
        195,
        116,
        68,
        144,
        117
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
          "name": "keeperSigner",
          "signer": true,
          "relations": [
            "state"
          ]
        },
        {
          "name": "bumpSigner"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        }
      ],
      "args": []
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
          "writable": true
        },
        {
          "name": "authority",
          "signer": true
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
          "name": "marginToken"
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
          "name": "indexTradeToken",
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
                "path": "indexTradeTokenIndex"
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
          "name": "params",
          "type": {
            "defined": {
              "name": "placeOrderParams"
            }
          }
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
          "name": "indexTradeTokenIndex",
          "type": "u16"
        }
      ]
    },
    {
      "name": "poolStake",
      "discriminator": [
        147,
        31,
        232,
        49,
        196,
        249,
        208,
        191
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
          "name": "poolMintVault",
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
                  109,
                  105,
                  110,
                  116,
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
          "name": "params",
          "type": {
            "defined": {
              "name": "stakeParams"
            }
          }
        },
        {
          "name": "poolIndex",
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
      "name": "poolUnStake",
      "discriminator": [
        206,
        229,
        122,
        214,
        2,
        66,
        133,
        2
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
          "name": "poolMintVault",
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
                  109,
                  105,
                  110,
                  116,
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
        },
        {
          "name": "poolIndex",
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
                "path": "marketIndex"
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
          "name": "params",
          "type": {
            "defined": {
              "name": "updatePositionLeverageParams"
            }
          }
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
          "name": "state"
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
      "name": "keyValue",
      "discriminator": [
        110,
        10,
        191,
        244,
        233,
        95,
        74,
        118
      ]
    },
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
      "name": "noMoreOrderSpace",
      "msg": "noMoreOrderSpace"
    },
    {
      "code": 6007,
      "name": "leverageIsNotAllowed",
      "msg": "leverageIsNotAllowed"
    },
    {
      "code": 6008,
      "name": "priceIsNotAllowed",
      "msg": "priceIsNotAllowed"
    },
    {
      "code": 6009,
      "name": "balanceNotEnough",
      "msg": "balanceNotEnough"
    },
    {
      "code": 6010,
      "name": "pythOffline",
      "msg": "pythOffline"
    },
    {
      "code": 6011,
      "name": "overflow",
      "msg": "overflow"
    },
    {
      "code": 6012,
      "name": "transferFailed",
      "msg": "transferFailed"
    },
    {
      "code": 6013,
      "name": "unableToLoadAccountLoader",
      "msg": "Unable to load AccountLoader"
    },
    {
      "code": 6014,
      "name": "cantPayUserInitFee",
      "msg": "cantPayUserInitFee"
    },
    {
      "code": 6015,
      "name": "couldNotFindUserToken",
      "msg": "couldNotFindUserToken"
    },
    {
      "code": 6016,
      "name": "couldNotFindUserOrder",
      "msg": "couldNotFindUserOrder"
    },
    {
      "code": 6017,
      "name": "couldNotFindUserPosition",
      "msg": "couldNotFindUserPosition"
    },
    {
      "code": 6018,
      "name": "onlyLiquidateIsolatePosition",
      "msg": "onlyLiquidateIsolatePosition"
    },
    {
      "code": 6019,
      "name": "onlyIsolatePositionAllowed",
      "msg": "onlyIsolatePositionAllowed"
    },
    {
      "code": 6020,
      "name": "couldNotFindUserStake",
      "msg": "couldNotFindUserStake"
    },
    {
      "code": 6021,
      "name": "oracleNotFound",
      "msg": "oracleNotFound"
    },
    {
      "code": 6022,
      "name": "oraclePriceToOld",
      "msg": "oraclePriceToOld"
    },
    {
      "code": 6023,
      "name": "unableToLoadOracle",
      "msg": "Unable To Load Oracles"
    },
    {
      "code": 6024,
      "name": "invalidOracle",
      "msg": "invalidOracle"
    },
    {
      "code": 6025,
      "name": "bnConversionError",
      "msg": "Conversion to u128/u128 failed with an overflow or underflow"
    },
    {
      "code": 6026,
      "name": "mathError",
      "msg": "Math Error"
    },
    {
      "code": 6027,
      "name": "castingFailure",
      "msg": "Casting Failure"
    },
    {
      "code": 6028,
      "name": "couldNotLoadMarketData",
      "msg": "couldNotLoadMarketData"
    },
    {
      "code": 6029,
      "name": "invalidMarketAccount",
      "msg": "invalidMarketAccount"
    },
    {
      "code": 6030,
      "name": "marketWrongMutability",
      "msg": "marketWrongMutability"
    },
    {
      "code": 6031,
      "name": "failedUnwrap",
      "msg": "Failed Unwrap"
    },
    {
      "code": 6032,
      "name": "userNotEnoughValue",
      "msg": "User Not Enough Value"
    },
    {
      "code": 6033,
      "name": "amountZero",
      "msg": "amountZero"
    },
    {
      "code": 6034,
      "name": "couldNotLoadTradeTokenData",
      "msg": "couldNotLoadTradeTokenData"
    },
    {
      "code": 6035,
      "name": "couldNotLoadPoolData",
      "msg": "couldNotLoadPoolData"
    },
    {
      "code": 6036,
      "name": "invalidTradeTokenAccount",
      "msg": "invalidTradeTokenAccount"
    },
    {
      "code": 6037,
      "name": "invalidTokenAccount",
      "msg": "invalidTokenAccount"
    },
    {
      "code": 6038,
      "name": "invalidPoolAccount",
      "msg": "invalidPoolAccount"
    },
    {
      "code": 6039,
      "name": "tradeTokenNotFind",
      "msg": "canNotFindTradeToken"
    },
    {
      "code": 6040,
      "name": "stakePaused",
      "msg": "stakePaused"
    },
    {
      "code": 6041,
      "name": "stakeToSmall",
      "msg": "stakeToSmall"
    },
    {
      "code": 6042,
      "name": "unStakeTooSmall",
      "msg": "unStakeTooSmall"
    },
    {
      "code": 6043,
      "name": "unStakeTooLarge",
      "msg": "unStakeTooLarge"
    },
    {
      "code": 6044,
      "name": "positionSideNotSupport",
      "msg": "positionSideNotSupport"
    },
    {
      "code": 6045,
      "name": "rewardsNotFound",
      "msg": "rewardsNotFound"
    },
    {
      "code": 6046,
      "name": "userNotFound",
      "msg": "userNotFound"
    },
    {
      "code": 6047,
      "name": "couldNotLoadUserData",
      "msg": "couldNotLoadUserData"
    },
    {
      "code": 6048,
      "name": "poolSubUnsettleNotEnough",
      "msg": "poolSubUnsettleNotEnough"
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
            "name": "lastUpdate",
            "type": "u128"
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
      "name": "initializeStateParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "minOrderMarginUsd",
            "type": "u128"
          },
          {
            "name": "maxMaintenanceMarginRate",
            "type": "u128"
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
            "type": "u128"
          },
          {
            "name": "poolFeeRewardRatio",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "keyValue",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "key",
            "type": "string"
          },
          {
            "name": "value",
            "type": "string"
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
            "name": "marketIndex",
            "type": "u16"
          },
          {
            "name": "poolKey",
            "type": "pubkey"
          },
          {
            "name": "poolMint",
            "type": "pubkey"
          },
          {
            "name": "indexMint",
            "type": "pubkey"
          },
          {
            "name": "stablePoolKey",
            "type": "pubkey"
          },
          {
            "name": "stablePoolMint",
            "type": "pubkey"
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
            "name": "marketTradeConfig",
            "type": {
              "defined": {
                "name": "marketConfig"
              }
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
            "name": "maxLeverage",
            "type": "u128"
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
            "name": "maxLongOpenInterestCap",
            "type": "u128"
          },
          {
            "name": "maxShortOpenInterestCap",
            "type": "u128"
          },
          {
            "name": "longShortRatioLimit",
            "type": "u128"
          },
          {
            "name": "longShortOiBottomLimit",
            "type": "u128"
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
            "name": "lastUpdate",
            "type": "u128"
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
      "name": "orderSide",
      "repr": {
        "kind": "c"
      },
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
      "repr": {
        "kind": "c"
      },
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
      "repr": {
        "kind": "c"
      },
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
            "name": "isCrossMargin",
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
            "name": "size",
            "type": "u128"
          },
          {
            "name": "orderMargin",
            "type": "u128"
          },
          {
            "name": "leverage",
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
            "name": "placeTime",
            "type": "u128"
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
            "name": "poolMintVault",
            "type": "pubkey"
          },
          {
            "name": "poolName",
            "type": {
              "array": [
                "u8",
                32
              ]
            }
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
            "name": "poolConfig",
            "type": {
              "defined": {
                "name": "poolConfig"
              }
            }
          },
          {
            "name": "totalSupply",
            "type": "u128"
          },
          {
            "name": "poolStatus",
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
            "name": "poolMint",
            "type": "pubkey"
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
            "name": "miniStakeAmount",
            "type": "u128"
          },
          {
            "name": "miniUnStakeAmount",
            "type": "u128"
          },
          {
            "name": "poolLiquidityLimit",
            "type": "u128"
          },
          {
            "name": "stakeFeeRate",
            "type": "u128"
          },
          {
            "name": "unStakeFeeRate",
            "type": "u128"
          },
          {
            "name": "unSettleMintRatioLimit",
            "type": "u128"
          },
          {
            "name": "borrowingInterestRate",
            "type": "u128"
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
      "repr": {
        "kind": "c"
      },
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
      "repr": {
        "kind": "c"
      },
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
      "name": "stakeParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "requestTokenAmount",
            "type": "u128"
          },
          {
            "name": "portfolio",
            "type": "bool"
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
            "name": "keeperSigner",
            "type": "pubkey"
          },
          {
            "name": "bumpSignerNonce",
            "type": "u8"
          },
          {
            "name": "numberOfMarkets",
            "type": "u16"
          },
          {
            "name": "numberOfPools",
            "type": "u16"
          },
          {
            "name": "numberOfTradeTokens",
            "type": "u16"
          },
          {
            "name": "minOrderMarginUsd",
            "type": "u128"
          },
          {
            "name": "maxMaintenanceMarginRate",
            "type": "u128"
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
            "name": "minPrecisionMultiple",
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
            "type": "u128"
          },
          {
            "name": "stakingFeeRewardRatio",
            "type": "u128"
          },
          {
            "name": "poolFeeRewardRatio",
            "type": "u128"
          }
        ]
      }
    },
    {
      "name": "stopType",
      "repr": {
        "kind": "c"
      },
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
            "name": "discount",
            "type": "u128"
          },
          {
            "name": "liquidationFactor",
            "type": "u128"
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
            "name": "mint",
            "type": "pubkey"
          },
          {
            "name": "oracle",
            "type": "pubkey"
          },
          {
            "name": "tradeTokenVault",
            "type": "pubkey"
          },
          {
            "name": "tokenIndex",
            "type": "u16"
          },
          {
            "name": "decimals",
            "type": "u16"
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
            "name": "padding",
            "type": {
              "array": [
                "u8",
                3
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
            "name": "unStakeTokenAmount",
            "type": "u128"
          },
          {
            "name": "portfolio",
            "type": "bool"
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
            "name": "isCrossMargin",
            "type": "bool"
          },
          {
            "name": "leverage",
            "type": "u128"
          },
          {
            "name": "addMarginAmount",
            "type": "u128"
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
            "name": "userKey",
            "type": "pubkey"
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
            "name": "userKey",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "nextOrderId",
            "type": "u128"
          },
          {
            "name": "nextLiquidationId",
            "type": "u128"
          },
          {
            "name": "hold",
            "type": "u128"
          },
          {
            "name": "userTokens",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "userToken"
                  }
                },
                8
              ]
            }
          },
          {
            "name": "userStakes",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "userStake"
                  }
                },
                8
              ]
            }
          },
          {
            "name": "userPositions",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "userPosition"
                  }
                },
                8
              ]
            }
          },
          {
            "name": "userOrders",
            "type": {
              "array": [
                {
                  "defined": {
                    "name": "userOrder"
                  }
                },
                8
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
            "name": "leverage",
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
            "name": "time",
            "type": "u128"
          },
          {
            "name": "orderId",
            "type": "u128"
          },
          {
            "name": "marginMint",
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
            "name": "crossMargin",
            "type": "bool"
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
            "name": "padding",
            "type": {
              "array": [
                "u8",
                9
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
            "name": "leverage",
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
            "name": "lastUpdateTime",
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
            "name": "marginMint",
            "type": "pubkey"
          },
          {
            "name": "indexMint",
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
            "name": "isLong",
            "type": "bool"
          },
          {
            "name": "crossMargin",
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
                12
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
            "name": "token",
            "type": "pubkey"
          },
          {
            "name": "realisedRewardsTokenAmount",
            "type": "u128"
          },
          {
            "name": "openRewardsPerStakeToken",
            "type": "u128"
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
            "name": "amount",
            "type": "u128"
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
            "name": "userRewards",
            "type": {
              "defined": {
                "name": "userRewards"
              }
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          }
        ]
      }
    },
    {
      "name": "userStakeStatus",
      "repr": {
        "kind": "c"
      },
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
      "name": "userToken",
      "repr": {
        "kind": "c"
      },
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "tokenMint",
            "type": "pubkey"
          },
          {
            "name": "userTokenAccountKey",
            "type": "pubkey"
          },
          {
            "name": "amount",
            "type": "u128"
          },
          {
            "name": "usedAmount",
            "type": "u128"
          },
          {
            "name": "liability",
            "type": "u128"
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
                "name": "userTokenUpdateOrigin"
              }
            }
          }
        ]
      }
    },
    {
      "name": "userTokenStatus",
      "repr": {
        "kind": "c"
      },
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
      "name": "userTokenUpdateOrigin",
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
