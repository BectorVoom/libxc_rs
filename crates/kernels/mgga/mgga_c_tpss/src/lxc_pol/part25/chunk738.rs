//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 738/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk738<F: Float>(t4834: F, t835: F, t128: F, t2454: F, t3746: F, t4828: F, t4832: F, t285: F, t1425: F, t3765: F, t1424: F, t866: F) -> (F, F, F, F, F, F, F) {
    let t4835 = t835 * t4834;
    let t4836 = t128 * t4835;
    let t4838 = t2454 + F::new(0.11872222222222222222e-1) * t3746 - F::new(0.11872222222222222222e-1) * t4828 + F::new(0.35616666666666666666e-1) * t4832 - F::new(0.17808333333333333333e-1) * t4836;
    let t4840 = F::new(0.621814e-1) * t4838 * t285;
    let t4842 = F::new(2.0) * t3765 * t1425;
    let t4843 = t1424 * t1424;
    let t4844 = t4843 * t866;
    (t4835, t4836, t4838, t4840, t4842, t4843, t4844)
}
