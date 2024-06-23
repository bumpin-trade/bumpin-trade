import {OracleClient} from "./oracles/types";
import {PublicKey} from '@solana/web3.js';
import {AccountSubscriber} from "./account/types";
import {State, UserAccount} from "./types";
import {BumpinClientConfig} from "./bumpinClientConfig";
import {PollingPoolAccountSubscriber} from "./account/pollingPoolAccountSubscriber";

export class Pool {
    // oracleClient: OracleClient
    // poolAccountSubscriber: AccountSubscriber<Pool>
    // state: State
    //
    // constructor(poolPublicKey: PublicKey, clientConfig: BumpinClientConfig) {
    //     this.oracleClient = clientConfig.oracleClient;
    //     this.poolAccountSubscriber = new PollingPoolAccountSubscriber(clientConfig.program, poolPublicKey, clientConfig.bulkAccountLoader);
    // }
}