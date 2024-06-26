export class BumpinClientNotInitialized extends Error {
    constructor() {
        super("Bumpin client not initialized");
    }
}

export class BumpinClientInternalError extends Error {
    constructor(msg: string = "") {
        super("Bumpin client internal error: " + msg);
    }
}

export class BumpinAccountNotFound extends Error {
    accountName: string;

    constructor(accountName: string) {
        super(`Account not found: ${accountName}`);
        this.accountName = accountName;

    }

    public getAccountName(): string {
        return this.accountName;
    }
}

export class BumpinSubscriptionFailed extends Error {
    accountName: string;
    index: number

    constructor(accountName: string, index: number = undefined) {
        super(`Account not subscribed: ${accountName}`);
        this.accountName = accountName;
        this.index = index
    }

    public getAccountName(): string {
        return this.accountName;
    }

    public getIndex(): number | undefined {
        return this.index;
    }
}