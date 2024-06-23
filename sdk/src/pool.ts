import {OracleClient} from "./oracles/types";
import {UserAccountSubscriber} from "./account/types";
import {UserAccount} from "./types";
import {BumpinClientConfig} from "./bumpinClientConfig";
import {PollingPoolAccountSubscriber} from "./account/pollingPoolAccountSubscriber";

export class Pool {
    oracleClient: OracleClient;
    userAccountSubscriber: UserAccountSubscriber<Pool>;

    constructor(clientConfig: BumpinClientConfig) {
        this.oracleClient = clientConfig.oracleClient;
        this.userAccountSubscriber = new PollingPoolAccountSubscriber(clientConfig.program,
            clientConfig.userAccountPublicKey, clientConfig.bulkAccountLoader);
        await this.userAccountSubscriber.subscribe();
    }
}