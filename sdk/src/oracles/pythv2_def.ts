import { Connection, PublicKey } from '@solana/web3.js';
import { Buffer } from 'buffer';

export type VerificationLevel =
    | { kind: 'Partial'; numSignatures: number }
    | { kind: 'Full' };

export interface PriceFeedMessage {
    feedId: Uint8Array;
    price: bigint;
    conf: bigint;
    exponent: number;
    publishTime: bigint;
    prevPublishTime: bigint;
    emaPrice: bigint;
    emaConf: bigint;
}

export interface PriceUpdateV2 {
    writeAuthority: PublicKey;
    verificationLevel: VerificationLevel;
    priceMessage: PriceFeedMessage;
    postedSlot: bigint;
}

function deserializePriceUpdateV2(buffer: Buffer): PriceUpdateV2 {
    let offset = 0;

    const readPublicKey = (): PublicKey => {
        const publicKey = new PublicKey(buffer.slice(offset, offset + 32));
        offset += 32;
        return publicKey;
    };

    const readBigInt64 = (): bigint => {
        const value = buffer.readBigInt64LE(offset);
        offset += 8;
        return value;
    };

    const readUint8Array = (length: number): Uint8Array => {
        const value = buffer.slice(offset, offset + length);
        offset += length;
        return new Uint8Array(value);
    };

    const readVerificationLevel = (): VerificationLevel => {
        const kind = buffer.readUInt8(offset);
        offset += 1;
        if (kind === 0) {
            const numSignatures = buffer.readUInt8(offset);
            offset += 1;
            return { kind: 'Partial', numSignatures };
        }
        return { kind: 'Full' };
    };

    const writeAuthority = readPublicKey();
    const verificationLevel = readVerificationLevel();
    const feedId = readUint8Array(32);
    const price = readBigInt64();
    const conf = readBigInt64();
    const exponent = buffer.readInt32LE(offset);
    offset += 4;
    const publishTime = readBigInt64();
    const prevPublishTime = readBigInt64();
    const emaPrice = readBigInt64();
    const emaConf = readBigInt64();
    const priceMessage: PriceFeedMessage = {
        feedId,
        price,
        conf,
        exponent,
        publishTime,
        prevPublishTime,
        emaPrice,
        emaConf,
    };
    const postedSlot = readBigInt64();

    return {
        writeAuthority,
        verificationLevel,
        priceMessage,
        postedSlot,
    };
}

// for test use
// const connection = new Connection('https://api.devnet.solana.com', 'confirmed');
// const publicKey = new PublicKey('4cSM2e6rvbGQUFiJbqytoVMi5GgghSMr8LwVrT9VPSPo');

async function fetchPriceUpdateV2ByAccount(
    connection: Connection,
    publicKey: PublicKey,
): Promise<PriceUpdateV2 | undefined> {
    const accountInfo = await connection.getAccountInfo(publicKey);
    if (accountInfo === null) {
        throw new Error('Account not found');
    }

    // must be the Pyth V2 receiver program, no matter what network is used
    if (
        !accountInfo.owner.equals(
            new PublicKey('rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ'),
        )
    ) {
        throw new Error('Invalid owner');
    }

    const buffer = accountInfo.data;
    return deserializePriceUpdateV2(buffer.slice(8));
}

export { fetchPriceUpdateV2ByAccount };