export interface AccountSubscriber<T> {
    isSubscribed: boolean;

    subscribe(userAccount?: T): Promise<boolean>;

    fetch(): Promise<void>;

    updateData(userAccount: T, slot: number): void;

    unsubscribe(): Promise<void>;

    getAccountAndSlot(): DataAndSlot<T>;
}

export type BufferAndSlot = {
    slot: number;
    buffer: Buffer | undefined;
};

export type DataAndSlot<T> = {
    data: T;
    slot: number;
};

export type ResubOpts = {
    resubTimeoutMs?: number;
    logResubMessages?: boolean;
};
