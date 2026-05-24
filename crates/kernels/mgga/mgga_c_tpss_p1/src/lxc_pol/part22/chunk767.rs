//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 767/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk767<F: Float>(t141: F, t4098: F, t1038: F, t4056: F, t2836: F, t2880: F, t2892: F, t2893: F, t4044: F, t4049: F, t4054: F, t4058: F, t4072: F, t4080: F, t4088: F, t4090: F, t4093: F, t4096: F) -> (F, F, F, F) {
    let t4099 = t141 * t4098;
    let t4101 = t1038 * t4056;
    let t4102 = t141 * t4101;
    let t4104 = -F::new(0.9494625e0) * t4072 + F::new(0.1898925e1) * t4080 + t2880 - F::cast_from(0.99655555555555555557e-1_f64) * t2836 - F::cast_from(0.99655555555555555557e-1_f64) * t4044 - F::cast_from(0.19931111111111111111e0_f64) * t4049 + F::cast_from(0.59793333333333333334e0_f64) * t4054 + F::cast_from(0.29896666666666666667e0_f64) * t4058 + F::new(0.15358125e0) * t4088 + F::new(0.3071625e0) * t4090 + t2892 - F::cast_from(0.54771111111111111111e-1_f64) * t2893 - F::cast_from(0.54771111111111111111e-1_f64) * t4093 - F::cast_from(0.27385555555555555556e-1_f64) * t4096 + F::cast_from(0.16431333333333333333e0_f64) * t4099 + F::cast_from(0.82156666666666666667e-1_f64) * t4102;
    (t4099, t4101, t4102, t4104)
}
