//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1047/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1047<F: Float>(t22055: F, t3440: F, t20234: F, t3441: F, t1177: F, t21745: F, t4900: F, t15390: F, t18469: F, t18416: F, t4904: F, t18409: F, t4919: F) -> (F, F, F, F, F, F) {
    let t22056 = t3440 * t22055;
    let t22059 = t3441 * t20234;
    let t22060 = t1177 * t22059;
    let t22063 = t4900 * t21745;
    let t22066 = t15390 * t18469;
    let t22069 = t18416 * t4904;
    let t22072 = t4919 * t18409;
    (t22056, t22060, t22063, t22066, t22069, t22072)
}
