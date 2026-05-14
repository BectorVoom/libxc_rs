//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 644/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk644<F: Float>(t2895: F, t4047: F, t141: F, t1038: F, t4052: F, t4056: F, t2836: F, t2880: F, t2892: F, t2893: F, t4044: F, t4049: F, t4054: F, t4058: F, t4072: F, t4080: F, t4088: F, t4090: F, t4093: F) -> (F, F, F, F, F, F, F) {
    let t4095 = t2895 * t4047;
    let t4096 = t141 * t4095;
    let t4098 = t1038 * t4052;
    let t4099 = t141 * t4098;
    let t4101 = t1038 * t4056;
    let t4102 = t141 * t4101;
    let t4104 = -0.9494625e0 * t4072 + 0.1898925e1 * t4080 + t2880 - 0.99655555555555555557e-1 * t2836 - 0.99655555555555555557e-1 * t4044 - 0.19931111111111111111e0 * t4049 + 0.59793333333333333334e0 * t4054 + 0.29896666666666666667e0 * t4058 + 0.15358125e0 * t4088 + 0.3071625e0 * t4090 + t2892 - 0.54771111111111111111e-1 * t2893 - 0.54771111111111111111e-1 * t4093 - 0.27385555555555555556e-1 * t4096 + 0.16431333333333333333e0 * t4099 + 0.82156666666666666667e-1 * t4102;
    (t4095, t4096, t4098, t4099, t4101, t4102, t4104)
}
