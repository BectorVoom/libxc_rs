//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2484/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2484<F: Float>(t135: F, t21537: F, t973: F, t21541: F, t21545: F, t13995: F, t18041: F, t10390: F, t1041: F, t21570: F, t2979: F, t4582: F, t48496: F, t49984: F, t5909: F, t62418: F, t68458: F, t68466: F, t68470: F, t68543: F, t68547: F, t68554: F, t70330: F, t977: F) -> (F, F, F, F) {
    let t70655 = t973 * t135 * t21537;
    let t70660 = t973 * t135 * t21541;
    let t70665 = t973 * t135 * t21545;
    let t70703 = t13995 * t18041;
    let t70707 = t973 * t2979 * t68470 / F::new(72.0) + t973 * t2979 * t68466 / F::new(72.0) + F::new(55.0) / F::new(15552.0) * t1041 * t4582 * t48496 * t70330 + t62418 / F::new(1152.0) - t973 * t977 * t68543 / F::new(12.0) + t973 * t977 * t68547 / F::new(16.0) - t973 * t977 * t68554 / F::new(48.0) - t973 * t977 * t68458 / F::new(48.0) - t49984 * t5909 / F::new(144.0) + t70703 / F::new(1152.0) + F::new(5.0) / F::new(4608.0) * t10390 * t21570;
    (t70655, t70660, t70665, t70707)
}
