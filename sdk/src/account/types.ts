export interface AccountSubscriber<T> {
    dataAndSlot?: DataAndSlot<T>;

    subscribe(onChange: (data: T) => void): Promise<void>;

    fetch(): Promise<void>;

    unsubscribe(): Promise<void>;

    setData(userAccount: T, slot?: number): void;
}

export interface UserAccountSubscriber<T> {
    isSubscribed: boolean;

    subscribe(userAccount?: T): Promise<boolean>;

    fetch(): Promise<void>;

    updateData(userAccount: T, slot: number): void;

    unsubscribe(): Promise<void>;

    getUserAccountAndSlot(): DataAndSlot<T>;
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