pub mod pyth_program {
    use anchor_lang::prelude::*;
    #[cfg(feature = "mainnet-beta")]
    declare_id!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");
    #[cfg(not(feature = "mainnet-beta"))]
    declare_id!("AszCjfpwoxuCy4wiHVo5R4sHFuAzp4bDEgYC1VC5jHT8");
}

pub mod switchboard_program {
    use anchor_lang::prelude::*;
    declare_id!("SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f");
}

pub mod bonk_oracle {
    use anchor_lang::prelude::*;
    #[cfg(feature = "mainnet-beta")]
    declare_id!("8ihFLu5FimgTQ1Unh4dVyEHUGodJ5gJQCrQf4KUVB9bN");
    #[cfg(not(feature = "mainnet-beta"))]
    declare_id!("6bquU99ktV1VRiHDr8gMhDFt3kMfhCQo5nfNrg2Urvsn");
}

pub mod pepe_oracle {
    use anchor_lang::prelude::*;
    #[cfg(feature = "mainnet-beta")]
    declare_id!("FSfxunDmjjbDV2QxpyxFCAPKmYJHSLnLuvQXDLkMzLBm");
    #[cfg(not(feature = "mainnet-beta"))]
    declare_id!("Gz9RfgDeAFSsH7BHDGyNTgCik74rjNwsodJpsCizzmkj");
}

pub mod usdc_oracle {
    use anchor_lang::prelude::*;
    #[cfg(feature = "mainnet-beta")]
    declare_id!("Gnt27xtC473ZT2Mw5u8wZ68Z3gULkSTb5DuxJy7eJotD");
    #[cfg(not(feature = "mainnet-beta"))]
    declare_id!("5SSkXsEKQepHHAewytPVwdej4epN1nxgLVM84L4KXgy7");
}

pub mod serum_program {
    use anchor_lang::prelude::*;
    #[cfg(feature = "mainnet-beta")]
    declare_id!("srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX");
    #[cfg(not(feature = "mainnet-beta"))]
    declare_id!("DESVgJVGajEgKGXhb6XmqDHGz3VjdgP7rEVESBgxmroY");
}

pub mod srm_mint {
    use anchor_lang::prelude::*;
    declare_id!("SRMuApVNdxXokk5GT7XD5cUUgXMBCoAz2LHeuAoKWRt");
}

pub mod msrm_mint {
    use anchor_lang::prelude::*;
    declare_id!("MSRMcoVyrFxnSgo5uXwone5SKcGhT1KEJMFEkMEWf9L");
}

pub mod jupiter_mainnet_6 {
    use anchor_lang::prelude::*;
    declare_id!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");
}
pub mod jupiter_mainnet_4 {
    use anchor_lang::prelude::*;
    declare_id!("JUP4Fb2cqiRUcaTHdrPC8h2gNsA2ETXiPDD33WcGuJB");
}
pub mod jupiter_mainnet_3 {
    use anchor_lang::prelude::*;
    declare_id!("JUP3c2Uh3WA4Ng34tw6kPd2G4C5BB21Xo36Je1s32Ph");
}

pub mod marinade_mainnet {
    use anchor_lang::prelude::*;
    declare_id!("MarBmsSgKXdrN1egZf5sqe1TMai9K1rChYNDJgjq7aD");
}

pub mod usdt_oracle_mainnet {
    use anchor_lang::prelude::*;
    declare_id!("3vxLXJqLqF3JG5TCbYycbKWRBbCJQLxQmBGCkyqEEefL");
}
