//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1292/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1292<F: Float>(t13133: F, t6113: F, t13554: F, t1600: F, t19596: F, t626: F, t19574: F, t6243: F, t19577: F, t6275: F, t19585: F, t21018: F, t5706: F, t1760: F, t21017: F, t23794: F) -> (F, F, F, F, F, F, F, F) {
    let t69392 = 4.0 * t13133 * t6113;
    let t69394 = 4.0 * t13554 * t6113;
    let t69397 = 4.0 * t626 * t1600 * t19596;
    let t69401 = 2.0 * t6243 * t19574;
    let t69403 = 2.0 * t19577 * t6275;
    let t69420 = 2.0 * t6243 * t19585;
    let t69422 = 6.0 * t5706 * t21018;
    let t69427 = 6.0 * t1760 * t23794 * t21017;
    (t69392, t69394, t69397, t69401, t69403, t69420, t69422, t69427)
}
