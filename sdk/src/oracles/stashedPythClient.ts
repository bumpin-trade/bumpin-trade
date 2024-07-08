import {parsePriceData} from '@pythnetwork/client';
import {PublicKey} from '@solana/web3.js';
import {OraclePriceData} from './types';
import {BN} from '@coral-xyz/anchor';
import {PollingPythAccountSubscriber} from "../account/pollingPythAccountSubscriber";
import {BulkAccountLoader} from "../account/bulkAccountLoader";
import {TEN} from "../constants/numericConstants";

export const PRICE_PRECISION = new BN(10).pow(new BN(8));
export class StashedPythClient {
    private readonly priceDataAccountPublicKey: PublicKey;
    private readonly stashLength: number;
    private queue: FixedLengthQueue<OraclePriceData>;
    private subscriber: PollingPythAccountSubscriber;

    public constructor(
        priceDataAccountPublicKey: PublicKey,
        stashLength: number,
        accountLoader: BulkAccountLoader
    ) {
        this.priceDataAccountPublicKey = priceDataAccountPublicKey;
        this.stashLength = stashLength;
        this.queue = new FixedLengthQueue<OraclePriceData>(stashLength);
        this.subscriber = new PollingPythAccountSubscriber(priceDataAccountPublicKey, accountLoader);
    }

    public getPriceDataAccountPublicKey(): PublicKey {
        return this.priceDataAccountPublicKey;
    }

    public getStashLength(): number {
        return this.stashLength;
    }

    public async initialize(): Promise<void> {
        await this.subscriber.subscribe((data: Buffer) => {
            const priceData = this.getOraclePriceDataFromBuffer(data);
            this.queue.enqueue(priceData);

        });
    }

    public getLastOraclePriceData(count: number): OraclePriceData[] {
        return this.queue.last(count).reverse();
    }

    public getLatestOraclePriceData(): OraclePriceData {
        return this.queue.last(1)[0];

    }

    public getOraclePriceDataFromBuffer(buffer: Buffer): OraclePriceData {
        const priceData = parsePriceData(buffer);
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
            twap: convertPythPrice(
                priceData.price,
                priceData.exponent,
            ),
            twapConfidence: convertPythPrice(
                priceData.price,
                priceData.exponent,
            ),
            hasSufficientNumberOfDataPoints: priceData.numQuoters >= minPublishers,
        };
    }
}

export function convertPythPrice(
    price: number,
    exponent: number,
): BN {
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

