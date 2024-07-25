pub mod pyth_program {
    use anchor_lang::prelude::*;
    #[cfg(feature = "mainnet-beta")]
    declare_id!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");
    #[cfg(feature = "testnet")]
    declare_id!("8tfDNiaEyrV6Q1U4DEXrEigs9DoDtkugzFbybENEbCDz");
    #[cfg(feature = "devnet")]
    declare_id!("gSbePebfvPy7tRqimPoVecS2UsBvYv46ynrzWocc92s");
    #[cfg(not(feature = "mainnet-beta"))]
    declare_id!("AQkVcL5spcyrqiKNJykGWGD78ry8Erkuub2t2ogUVWca");
}
