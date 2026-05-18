//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 770/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk770<F: Float>(t1062: F, t4142: F, t1530: F, t2957: F, t1061: F, t2836: F, t2961: F, t4044: F, t4049: F, t4054: F, t4058: F, t434: F) -> (F, F, F, F, F) {
    let t4143 = t4142 * t1062;
    let t4146 = t1530 * t2957;
    let t4147 = t4146 * t1061;
    let t4155 = t2961 - F::new(0.30902777777777777778e-2) * t2836 - F::new(0.30902777777777777778e-2) * t4044 - F::new(0.61805555555555555555e-2) * t4049 + F::new(0.18541666666666666667e-1) * t4054 + F::new(0.92708333333333333333e-2) * t4058;
    let t4156 = t4155 * t434;
    (t4143, t4146, t4147, t4155, t4156)
}
