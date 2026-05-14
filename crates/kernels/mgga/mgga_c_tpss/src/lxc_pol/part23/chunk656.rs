//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 656/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk656<F: Float>(t2840: F, t3032: F, t1985: F, t926: F, t1100: F, t2845: F, t1101: F, t1992: F, t1106: F, t451: F) -> (F, F, F, F, F, F, F) {
    let t3033 = t3032 * t2840;
    let t3034 = t3033 * t1985;
    let t3035 = t926 * t3034;
    let t3038 = t1100 * t2845;
    let t3039 = t3038 * t1985;
    let t3040 = t926 * t3039;
    let t3043 = t1101 * t1992;
    let t3044 = t926 * t3043;
    let t3048 = 1.0 / t1106 / t451;
    (t3034, t3035, t3039, t3040, t3043, t3044, t3048)
}
