import { parsePriceData } from '@pythnetwork/client';
import { Connection, PublicKey } from '@solana/web3.js';
import { OraclePriceData } from './types';
import { BN } from '@coral-xyz/anchor';
import { TEN } from '../constants/numericConstants';
import { BumpinInvalidParameter } from '../errors';
import { fetchPriceUpdateV2ByAccount, PriceUpdateV2 } from './pythv2_def';

export const PRICE_PRECISION = new BN(10).pow(new BN(8));

export class StashedPythV2Client {
    private readonly account: PublicKey;
    private readonly stashLength: number;
    private readonly connection: Connection;
    private queue: FixedLengthQueue<PriceUpdateV2>;
    private readonly interval: number;

    public constructor(
        account: PublicKey,
        stashLength: number,
        connection: Connection,
        interval: number = 1000,
    ) {
        this.account = account;
        this.stashLength = stashLength;
        this.queue = new FixedLengthQueue<PriceUpdateV2>(stashLength);
        this.connection = connection;
        this.interval = interval;
    }

    public async getPriceData(): Promise<PriceUpdateV2> {
        const priceUpdateV2: PriceUpdateV2 | undefined = await fetchPriceUpdateV2ByAccount(this.connection, this.account);
        if (!priceUpdateV2) {
            throw new BumpinInvalidParameter('Price data not found');
        }
        return priceUpdateV2;
    }

    public getPriceDataAccountPublicKey(): PublicKey {
        return this.account;
    }

    public getStashLength(): number {
        return this.stashLength;
    }

    public async initialize(): Promise<void> {
        setInterval(async () => {
            try {
                const priceUpdateV2: PriceUpdateV2 | undefined = await fetchPriceUpdateV2ByAccount(this.connection, this.account);
                if (!priceUpdateV2) {
                    return;
                }
                this.queue.enqueue(priceUpdateV2!);
            } catch (e) {
                console.error('Error in stashedPythV2Client: ', e);
            }
        }, this.interval);
    }

    public getLastOraclePriceData(count: number): PriceUpdateV2[] {
        let last = this.queue.last(count).reverse();
        if (last.length === 0) {
            throw new BumpinInvalidParameter('Price data not found');
        }
        return last;
    }

    public getLatestOraclePriceData(): PriceUpdateV2 {
        return this.queue.last(1)[0];
    }

    public getOraclePriceDataFromBuffer(buffer: Buffer): OraclePriceData {
        const priceData = parsePriceData(buffer);
        if (!priceData.confidence || !priceData.price) {
            throw new BumpinInvalidParameter('Price data not found');
        }

        const confidence = convertPythPrice(
            priceData.confidence,
            priceData.exponent,
        );
        const minPublishers = Math.min(priceData.numComponentPrices, 3);
        let price = convertPythPrice(
            priceData.aggregate.price,
            priceData.exponent,
        );

        return {
            price,
            slot: new BN(priceData.lastSlot.toString()),
            confidence,
            twap: convertPythPrice(priceData.price, priceData.exponent),
            twapConfidence: convertPythPrice(
                priceData.price,
                priceData.exponent,
            ),
            hasSufficientNumberOfDataPoints:
                priceData.numQuoters >= minPublishers,
        };
    }
}

export function convertPythPrice(price: number, exponent: number): BN {
    exponent = Math.abs(exponent);
    const pythPrecision = TEN.pow(new BN(exponent).abs());
    return new BN(price * Math.pow(10, exponent))
        .mul(PRICE_PRECISION)
        .div(pythPrecision);
}

class FixedLengthQueue<T> {
    private queue: T[];
    private maxLength: number;

    constructor(maxLength: number) {
        if (maxLength <= 0) {
            throw new Error('maxLength must be greater than 0');
        }
        this.queue = [];
        this.maxLength = maxLength;
    }

    getQueue(): T[] {
        return this.queue;
    }

    enqueue(item: T): void {
        if (this.queue.length >= this.maxLength) {
            this.queue.shift();
        }
        this.queue.push(item);
    }

    dequeue(): T | undefined {
        return this.queue.shift();
    }

    peek(): T | undefined {
        return this.queue[0];
    }

    last(count: number): T[] {
        return this.queue.slice(-count);
    }

    size(): number {
        return this.queue.length;
    }

    isEmpty(): boolean {
        return this.queue.length === 0;
    }

    isFull(): boolean {
        return this.queue.length === this.maxLength;
    }

    clear(): void {
        this.queue = [];
    }
}
