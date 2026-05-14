//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 532/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk532<F: Float>(t2697: F, t849: F, t1891: F, t241: F, t67: F, t225: F, t853: F, t257: F, t856: F, t68: F, t252: F, t2627: F, t814: F, t852: F, t261: F, t1878: F, t268: F, t271: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2698 = t2697 * t849;
    let t2700 = t241 * t1891;
    let t2701 = t2700 * t67;
    let t2713 = t853 * t225;
    let t2717 = 1.0 / t856 / t257;
    let t2718 = t68 * t2717;
    let t2728 = t2627 * t252;
    let t2732 = t814 * t852;
    let t2751 = t261 * t261;
    let t2752 = 1.0 / t2751;
    let t2764 = t268 * t1878 * t271;
    (t2698, t2701, t2713, t2718, t2728, t2732, t2751, t2752, t2764)
}
