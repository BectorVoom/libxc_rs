//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1336/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1336<F: Float>(t2840: F, t287: F, t275: F, t10294: F, t10544: F, t891: F, t2843: F, t290: F, t2860: F, t919: F, t2904: F, t938: F) -> (F, F, F, F, F, F, F) {
    let t10660 = F::cast_from(1.0_f64) / t2840 / t287;
    let t10661 = t275 * t10660;
    let t10675 = F::cast_from(0.36514074074074074075e0_f64) * t10294;
    let t10676 = F::cast_from(0.93011851851851851854e0_f64) * t10544;
    let t10701 = F::cast_from(1.0_f64) / t2840 / t891;
    let t10702 = t275 * t10701;
    let t10704 = F::cast_from(1.0_f64) / t2843 / t290;
    let t10740 = t919 * t2860;
    let t10747 = t938 * t2904;
    (t10661, t10675, t10676, t10702, t10704, t10740, t10747)
}
