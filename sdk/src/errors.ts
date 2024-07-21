import { BN } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

export class BumpinClientNotInitialized extends Error {
  whichComponent: string;

  constructor(whichComponent: string = "self") {
    super("Bumpin client not initialized for " + whichComponent);
    this.whichComponent = whichComponent;
    Object.setPrototypeOf(this, BumpinClientNotInitialized.prototype);
  }
}

export class BumpinClientInternalError extends Error {
  constructor(msg: string = "") {
    super("Bumpin client internal error: " + msg);
    Object.setPrototypeOf(this, BumpinClientInternalError.prototype);
  }
}

export class BumpinUserNotLogin extends Error {
  constructor() {
    super(`User not login`);
    Object.setPrototypeOf(this, BumpinUserNotLogin.prototype);
  }
}

export class BumpinPoolNotFound extends Error {
  mint: PublicKey;

  constructor(mint: PublicKey) {
    super(`Pool not found: ${mint.toString()}`);
    this.mint = mint;
    Object.setPrototypeOf(this, BumpinPoolNotFound.prototype);
  }

  public getMint(): PublicKey {
    return this.mint;
  }
}

export class BumpinMarketNotFound extends Error {
  symbol: number[];

  constructor(symbol: number[]) {
    super(`Pool not found: ${symbol}`);
    this.symbol = symbol;
    Object.setPrototypeOf(this, BumpinPoolNotFound.prototype);
  }

  public getSymbol(): number[] {
    return this.symbol;
  }
}

export class BumpinTokenNotFound extends Error {
  mint: PublicKey;

  constructor(mint: PublicKey) {
    super(`Token not found: ${mint}`);
    this.mint = mint;
    Object.setPrototypeOf(this, BumpinTokenNotFound.prototype);
  }

  public getMint(): PublicKey {
    return this.mint;
  }
}

export class BumpinInvalidParameter extends Error {
  constructor(msg: string) {
    super(`Invalid parameter: ${msg}`);
    Object.setPrototypeOf(this, BumpinInvalidParameter.prototype);
  }
}

export class BumpinSupplyInsufficient extends Error {
  minimalExpected: BN;
  actualValue: BN;

  constructor(minimalExpected: BN, actualValue: BN) {
    super(
      `Supply is insufficient: ${actualValue}  < ${minimalExpected} (expected)`
    );
    this.minimalExpected = minimalExpected;
    this.actualValue = actualValue;
    Object.setPrototypeOf(this, BumpinSupplyInsufficient.prototype);
  }

  public getMinimalExpected(): BN {
    return this.minimalExpected;
  }

  public getActualValue(): BN {
    return this.actualValue;
  }
}

export class BumpinValueInsufficient extends Error {
  minimalExpected: BN;
  actualValue: BN;

  constructor(minimalExpected: BN, actualValue: BN) {
    super(
      `Value is insufficient: ${actualValue}  < ${minimalExpected} (expected)`
    );
    this.minimalExpected = minimalExpected;
    this.actualValue = actualValue;
    Object.setPrototypeOf(this, BumpinValueInsufficient.prototype);
  }

  public getMinimalExpected(): BN {
    return this.minimalExpected;
  }

  public getActualValue(): BN {
    return this.actualValue;
  }
}

export class BumpinTokenAccountUnexpected extends Error {
  expected: string;
  actual: string;

  constructor(expected: string, actual: string) {
    super(`Token account unexpected: ${actual}  != ${expected} (expected)`);
    this.expected = expected;
    this.actual = actual;
    Object.setPrototypeOf(this, BumpinTokenAccountUnexpected.prototype);
  }

  public getExpected(): string {
    return this.expected;
  }

  public getActual(): string {
    return this.actual;
  }
}

export class BumpinAccountNotFound extends Error {
  accountName: string;

  constructor(accountName: string) {
    super(`Account not found: ${accountName}`);
    this.accountName = accountName;
    Object.setPrototypeOf(this, BumpinAccountNotFound.prototype);
  }

  public getAccountName(): string {
    return this.accountName;
  }
}

export class BumpinSubscriptionFailed extends Error {
  accountName: string;
  index: number;

  constructor(accountName: string, index: number = -1) {
    super(`Account not subscribed: ${accountName}`);
    this.accountName = accountName;
    this.index = index;
    Object.setPrototypeOf(this, BumpinSubscriptionFailed.prototype);
  }

  public getAccountName(): string {
    return this.accountName;
  }

  public getIndex(): number | undefined {
    return this.index;
  }
}
