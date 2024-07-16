import { parsePriceData, PriceData } from "@pythnetwork/client";
import { Connection, PublicKey } from "@solana/web3.js";
import { OracleClient, OraclePriceData } from "./types";
import { BN } from "@coral-xyz/anchor";
import { ONE, QUOTE_PRECISION, TEN } from "../constants/numericConstants";
import { BumpinInvalidParameter } from "../errors";

export const PRICE_PRECISION = new BN(10).pow(new BN(8));

export class PythClient implements OracleClient {
  private connection: Connection;
  private multiple: BN;
  private stableCoin: boolean;

  public constructor(
    connection: Connection,
    multiple = ONE,
    stableCoin = false
  ) {
    this.connection = connection;
    this.multiple = multiple;
    this.stableCoin = stableCoin;
  }

  public async getPriceData(pricePublicKey: PublicKey): Promise<PriceData> {
    const accountInfo = await this.connection.getAccountInfo(pricePublicKey);
    if (!accountInfo) {
      throw new BumpinInvalidParameter(
        "Price account not found, pricePublicKey: " + pricePublicKey.toString()
      );
    }
    return parsePriceData(accountInfo.data);
  }

  public async getOraclePriceData(
    pricePublicKey: PublicKey
  ): Promise<OraclePriceData> {
    const accountInfo = await this.connection.getAccountInfo(pricePublicKey);
    if (!accountInfo) {
      throw new BumpinInvalidParameter(
        "Price account not found, pricePublicKey: " + pricePublicKey.toString()
      );
    }
    return this.getOraclePriceDataFromBuffer(accountInfo.data);
  }

  public getOraclePriceDataFromBuffer(buffer: Buffer): OraclePriceData {
    const priceData = parsePriceData(buffer);

    if (!priceData.price) {
      throw new BumpinInvalidParameter("Price data not found");
    }

    const confidence = convertPythPrice(
      priceData.confidence ?? 0,
      priceData.exponent,
      this.multiple
    );
    const minPublishers = Math.min(priceData.numComponentPrices, 3);
    let price = convertPythPrice(
      priceData.aggregate.price,
      priceData.exponent,
      this.multiple
    );
    if (this.stableCoin) {
      price = getStableCoinPrice(price, confidence);
    }

    return {
      price,
      slot: new BN(priceData.lastSlot.toString()),
      confidence,
      twap: convertPythPrice(
        priceData.price,
        priceData.exponent,
        this.multiple
      ),
      twapConfidence: convertPythPrice(
        priceData.price,
        priceData.exponent,
        this.multiple
      ),
      hasSufficientNumberOfDataPoints: priceData.numQuoters >= minPublishers,
    };
  }
}

export function convertPythPrice(
  price: number,
  exponent: number,
  multiple: BN
): BN {
  exponent = Math.abs(exponent);
  const pythPrecision = TEN.pow(new BN(exponent).abs()).div(multiple);
  return new BN(price * Math.pow(10, exponent))
    .mul(PRICE_PRECISION)
    .div(pythPrecision);
}

const fiveBPS = new BN(500);

function getStableCoinPrice(price: BN, confidence: BN): BN {
  if (price.sub(QUOTE_PRECISION).abs().lt(BN.min(confidence, fiveBPS))) {
    return QUOTE_PRECISION;
  } else {
    return price;
  }
}
