//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 661/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk661<F: Float>(t1053: F, t1523: F, t1061: F, t1531: F, t2836: F, t2893: F, t2937: F, t2944: F, t4044: F, t4049: F, t4054: F, t4058: F, t4072: F, t4080: F, t4088: F, t4090: F, t4093: F, t4096: F, t4099: F, t4102: F) -> (F, F, F) {
    let t4120 = t1523 * t1053;
    let t4125 = t1531 * t1061;
    let t4142 = -F::new(0.17648625e1) * t4072 + F::new(0.3529725e1) * t4080 + t2937 - F::cast_from(0.17215833333333333333e0_f64) * t2836 - F::cast_from(0.17215833333333333333e0_f64) * t4044 - F::cast_from(0.34431666666666666667e0_f64) * t4049 + F::new(0.103295e1) * t4054 + F::new(0.516475e0) * t4058 + F::new(0.31558125e0) * t4088 + F::new(0.6311625e0) * t4090 + t2944 - F::cast_from(0.69463333333333333333e-1_f64) * t2893 - F::cast_from(0.69463333333333333333e-1_f64) * t4093 - F::cast_from(0.34731666666666666667e-1_f64) * t4096 + F::new(0.20839e0) * t4099 + F::new(0.104195e0) * t4102;
    (t4120, t4125, t4142)
}
