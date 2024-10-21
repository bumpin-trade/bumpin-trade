pub mod pyth_program {
    use anchor_lang::prelude::*;

    #[cfg(feature = "localnet")]
    declare_id!("FsJ3A3u2vn5cTVofAjvy6y5kwABJAqYWpe4975bi2epH");
    #[cfg(not(feature = "localnet"))]
    declare_id!("rec5EKMGg6MxZYaMdyBfgwp4d5rB9T1VQH5pJv5LtFJ");
}
