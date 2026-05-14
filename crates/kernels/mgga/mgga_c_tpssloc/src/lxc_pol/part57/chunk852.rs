//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 852/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk852<F: Float>(t1845: F, t7752: F, t120179: F, t1992: F, t32693: F, t90566: F, t22635: F, t31090: F, t6460: F, t22685: F, t28191: F, t31137: F, t120317: F, t1842: F, t1985: F, t28232: F) -> (F, F, F, F, F, F, F) {
    let t127162 = t1845 * t7752;
    let t127166 = 0.15352717957250113407e0 * t120179;
    let t127169 = 0.6579736267392905746e-1 * t1992 * t90566 * t32693;
    let t127173 = 0.3289868133696452873e-1 * t1992 * t22635 * t31090 * t6460;
    let t127176 = 0.9869604401089358619e-1 * t22685 * t31137 * t28191;
    let t127180 = 0.6579736267392905746e-1 * t1992 * t22635 * t120317 * t1842;
    let t127183 = 0.3289868133696452873e-1 * t1985 * t31137 * t28232;
    (t127162, t127166, t127169, t127173, t127176, t127180, t127183)
}
