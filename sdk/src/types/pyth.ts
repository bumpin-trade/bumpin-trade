/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/pyth.json`.
 */
export type Pyth = {
    address: 'ELu6XkLaZ9Csj1UMWfqGxwxTsReLFCdJamxzLJfKKbyH';
    metadata: {
        name: 'pyth';
        version: '0.1.0';
        spec: '0.1.0';
    };
    instructions: [
        {
            name: 'initialize';
            discriminator: [175, 175, 109, 31, 13, 152, 155, 237];
            accounts: [
                {
                    name: 'price';
                    writable: true;
                },
            ];
            args: [
                {
                    name: 'price';
                    type: 'i64';
                },
                {
                    name: 'exponent';
                    type: 'i32';
                },
                {
                    name: 'conf';
                    type: 'u64';
                },
            ];
        },
        {
            name: 'initializeV2';
            discriminator: [67, 153, 175, 39, 218, 16, 38, 32];
            accounts: [
                {
                    name: 'priceUpdateV2';
                    writable: true;
                },
            ];
            args: [
                {
                    name: 'params';
                    type: {
                        defined: {
                            name: 'initializeV2Params';
                        };
                    };
                },
            ];
        },
        {
            name: 'setPrice';
            discriminator: [16, 19, 182, 8, 149, 83, 72, 181];
            accounts: [
                {
                    name: 'price';
                    writable: true;
                },
            ];
            args: [
                {
                    name: 'price';
                    type: 'i64';
                },
                {
                    name: 'conf';
                    type: 'u64';
                },
            ];
        },
    ];
    types: [
        {
            name: 'initializeV2Params';
            type: {
                kind: 'struct';
                fields: [
                    {
                        name: 'feedId';
                        type: {
                            array: ['u8', 32];
                        };
                    },
                    {
                        name: 'price';
                        type: 'i64';
                    },
                    {
                        name: 'exponent';
                        type: 'i32';
                    },
                ];
            };
        },
    ];
};
