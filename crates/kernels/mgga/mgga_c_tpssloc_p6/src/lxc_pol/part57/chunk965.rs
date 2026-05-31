//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 965/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk965<F: Float>(t28237: F, t3701: F, t32673: F, t32675: F, t32678: F, t1845: F, t7752: F, t120179: F, t1992: F, t32693: F, t90566: F, t22635: F, t31090: F, t6460: F) -> (F, F, F, F, F, F, F, F) {
    let t127114 = t3701 * t28237;
    let t127122 = F::cast_from(4.0_f64) * t32673;
    let t127124 = F::cast_from(4.0_f64) * t32675;
    let t127125 = F::cast_from(4.0_f64) * t32678;
    let t127162 = t1845 * t7752;
    let t127166 = F::cast_from(0.15352717957250113407e0_f64) * t120179;
    let t127169 = F::cast_from(0.6579736267392905746e-1_f64) * t1992 * t90566 * t32693;
    let t127173 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t22635 * t31090 * t6460;
    (t127114, t127122, t127124, t127125, t127162, t127166, t127169, t127173)
}
