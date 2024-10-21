pub mod pyth_program {
    use anchor_lang::prelude::*;

    #[cfg(feature = "localnet")]
    declare_id!("ECKhW7wvKQGGhzGFS7LqGv4z3DRoAD8HJywd25XjBoxP");
    #[cfg(not(feature = "localnet"))]
    declare_id!("ECKhW7wvKQGGhzGFS7LqGv4z3DRoAD8HJywd25XjBoxP");
}
