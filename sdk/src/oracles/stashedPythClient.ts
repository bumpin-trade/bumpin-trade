import { parsePriceData, PriceData } from '@pythnetwork/client';
import { PublicKey } from '@solana/web3.js';
import { OraclePriceData } from './types';
import { BN } from '@coral-xyz/anchor';
import { PollingPythAccountSubscriber } from '../account/pollingPythAccountSubscriber';
import { BulkAccountLoader } from '../account/bulkAccountLoader';
import { TEN } from '../constants/numericConstants';
import { BumpinInvalidParameter } from '../errors';

export const PRICE_PRECISION = new BN(10).pow(new BN(8));

export class StashedPythClient {
    private readonly priceDataAccountPublicKey: PublicKey;
    private readonly stashLength: number;
    private queue: FixedLengthQueue<PriceData>;
    private subscriber: PollingPythAccountSubscriber;

    public constructor(
        priceDataAccountPublicKey: PublicKey,
        stashLength: number,
        accountLoader: BulkAccountLoader,
    ) {
        this.priceDataAccountPublicKey = priceDataAccountPublicKey;
        this.stashLength = stashLength;
        this.queue = new FixedLengthQueue<PriceData>(stashLength);
        this.subscriber = new PollingPythAccountSubscriber(
            priceDataAccountPublicKey,
            accountLoader,
        );
    }

    public getPriceDataAccountPublicKey(): PublicKey {
        return this.priceDataAccountPublicKey;
    }

    public getStashLength(): number {
        return this.stashLength;
    }

    public async initialize(): Promise<void> {
        await this.subscriber.subscribe((data: Buffer) => {
            let priceData = parsePriceData(data);
            this.queue.enqueue(priceData);
            // console.log('Price data updated, key: ', this.priceDataAccountPublicKey.toString(), 'price: ', priceData.price, 'queue size: ', this.queue.size(),
            //     'price 1: ' , this.queue.getQueue()[0].price.toString(), 'price 2: ', this.queue.getQueue()[1].price.toString());
        });
    }

    public getLastOraclePriceData(count: number): PriceData[] {
        let last = this.queue.last(count).reverse();
        if (last.length === 0) {
            throw new BumpinInvalidParameter('Price data not found');
        }
        return last;
    }

    public getLatestOraclePriceData(): PriceData {
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
