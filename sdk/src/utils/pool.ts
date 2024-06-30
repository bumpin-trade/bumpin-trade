import {PublicKey} from "@solana/web3.js";
import {Pool} from "../types";
import {BumpinPoolNotFound} from "../errors";

export class BumpinPoolUtils {
    public static getPoolByMintPublicKey(mint: PublicKey, pools: Pool[]): Pool {
        let pool = pools.find((pool) => {
            return pool.mintKey.equals(mint);
        });
        if (pool === undefined) {
            throw new BumpinPoolNotFound(mint);
        }
        return pool;
    }
}