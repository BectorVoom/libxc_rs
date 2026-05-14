//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1346/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1346<F: Float>(t11931: F, t225: F, t11604: F, t496: F, t68: F, t3599: F, t11601: F, t11599: F, t11606: F, t11608: F, t11613: F, t11868: F, t1190: F, t11919: F, t11925: F, t11928: F, t11935: F, t1238: F, t1252: F, t3487: F, t3593: F, t3600: F, t3630: F, t3631: F, t45314: F, t466: F, t498: F) -> (F,) {
    let t45345 = t11931 * t225;
    let t45349 = 1.0 / t11604 / t496;
    let t45350 = t68 * t45349;
    let t45351 = t3599 * t3599;
    let t45355 = t11601 * t225;
    let t45375 = t11599 * t225;
    let t45382 = -36.0 * t11606 * t1238 * t3599 * t3630 + 4.0 * t11868 * t1190 * t498 + 24.0 * t1238 * t45350 * t45351 + t45314 * t466 * t498 - 24.0 * t11608 * t3487 - 12.0 * t11613 * t3631 - 4.0 * t11919 * t3593 - 6.0 * t11925 * t3631 + 12.0 * t11928 * t3600 + 24.0 * t11935 * t3487 - 12.0 * t1252 * t45345 - 12.0 * t1252 * t45355 - 4.0 * t1252 * t45375;
    (t45382,)
}
