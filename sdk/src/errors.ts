export class BumpinClientNotInitialized extends Error {
    constructor() {
        super("Bumpin client not initialized");
    }
}

export class BumpinAccountNotFound extends Error {
    accountName: string;

    constructor(accountName: string) {
        super(`Account not found: ${accountName}`);
    }

    public getAccountName(): string {
        return this.accountName;
    }
}