pub mod pyth_program {
    use anchor_lang::prelude::*;
    #[cfg(feature = "mainnet-beta")]
    declare_id!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");
    #[cfg(not(feature = "mainnet-beta"))]
    declare_id!("AQkVcL5spcyrqiKNJykGWGD78ry8Erkuub2t2ogUVWca");
}
