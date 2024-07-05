pub mod pyth_program {
    use anchor_lang::prelude::*;
    #[cfg(feature = "mainnet-beta")]
    declare_id!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");
    #[cfg(not(feature = "mainnet-beta"))]
    declare_id!("AszCjfpwoxuCy4wiHVo5R4sHFuAzp4bDEgYC1VC5jHT8");
}
