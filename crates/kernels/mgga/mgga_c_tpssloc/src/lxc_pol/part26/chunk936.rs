//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 936/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk936<F: Float>(t11918: F, t1241: F, t11868: F, t466: F, t225: F, t3591: F, t3482: F, t1190: F, t3590: F, t1251: F, t3630: F, t3598: F, t11599: F, t11601: F, t11608: F, t11613: F, t1238: F, t1252: F, t3487: F, t3593: F, t3600: F, t3631: F, t498: F) -> (F, F, F, F, F, F, F, F) {
    let t11919 = t1241 * t11918;
    let t11923 = t466 * t11868;
    let t11925 = t3591 * t225;
    let t11928 = t3482 * t225;
    let t11931 = t1190 * t3590;
    let t11934 = t1251 * t3630;
    let t11935 = t3598 * t11934;
    let t11940 = t11599 * t498 + 3.0 * t11601 * t498 - 6.0 * t11608 * t1238 - 6.0 * t11613 * t1252 - t11919 * t1238 + t11923 * t498 - 3.0 * t11925 * t1252 - 3.0 * t11928 * t1252 + 3.0 * t11931 * t498 + 6.0 * t11935 * t1238 + 6.0 * t3487 * t3600 - 3.0 * t3487 * t3631 + 6.0 * t3593 * t3600 - 3.0 * t3593 * t3631;
    (t11919, t11923, t11925, t11928, t11931, t11934, t11935, t11940)
}
