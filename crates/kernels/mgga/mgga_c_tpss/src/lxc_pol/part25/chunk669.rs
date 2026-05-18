//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 669/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk669<F: Float>(t1113: F, t3054: F, t4231: F, t3931: F, t1562: F, t3060: F, t242: F, t1111: F, t1015: F, t3068: F, t4062: F, t4065: F, t4067: F, t4070: F, t4107: F, t4111: F, t4189: F, t4191: F, t4194: F, t4196: F, t4200: F, t4204: F, t4209: F) -> (F, F, F, F, F, F, F, F) {
    let t4232 = t3054 * t1113;
    let t4233 = t4231 * t4232;
    let t4234 = t3931 * t4233;
    let t4237 = t3060 * t1562;
    let t4238 = t242 * t4237;
    let t4239 = t1111 * t4238;
    let t4241 = t1562 * t1015;
    let t4242 = t3068 * t4241;
    let t4245 = -t4062 + t4065 + t4067 - t4070 + t4107 + t4111 + t4189 + t4191 - t4194 - t4196 + t4200 - t4204 - t4209;
    (t4232, t4233, t4234, t4238, t4239, t4241, t4242, t4245)
}
