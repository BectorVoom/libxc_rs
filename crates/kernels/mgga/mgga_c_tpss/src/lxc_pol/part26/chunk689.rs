//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 689/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk689<F: Float>(t242: F, t4237: F, t1111: F, t1015: F, t1562: F, t3068: F, t4062: F, t4065: F, t4067: F, t4070: F, t4107: F, t4111: F, t4189: F, t4191: F, t4194: F, t4196: F, t4200: F, t4204: F, t4209: F) -> (F, F, F, F, F) {
    let t4238 = t242 * t4237;
    let t4239 = t1111 * t4238;
    let t4241 = t1562 * t1015;
    let t4242 = t3068 * t4241;
    let t4245 = -t4062 + t4065 + t4067 - t4070 + t4107 + t4111 + t4189 + t4191 - t4194 - t4196 + t4200 - t4204 - t4209;
    (t4238, t4239, t4241, t4242, t4245)
}
