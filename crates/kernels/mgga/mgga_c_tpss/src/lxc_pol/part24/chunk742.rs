//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 742/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk742<F: Float>(t4834: F, t835: F, t128: F, t2454: F, t3746: F, t4828: F, t4832: F, t285: F, t1425: F, t3765: F, t1424: F, t866: F, t2481: F, t1415: F, t2487: F, t2491: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4835 = t835 * t4834;
    let t4836 = t128 * t4835;
    let t4838 = t2454 + 0.11872222222222222222e-1 * t3746 - 0.11872222222222222222e-1 * t4828 + 0.35616666666666666666e-1 * t4832 - 0.17808333333333333333e-1 * t4836;
    let t4840 = 0.621814e-1 * t4838 * t285;
    let t4842 = 2.0 * t3765 * t1425;
    let t4843 = t1424 * t1424;
    let t4844 = t4843 * t866;
    let t4846 = 2.0 * t2481 * t4844;
    let t4847 = t1415 * t1415;
    let t4848 = t2487 * t4847;
    let t4854 = t2491 + 2.0 / 9.0 * t3746 - 2.0 / 9.0 * t4828 + 2.0 / 3.0 * t4832 - t4836 / 3.0;
    (t4835, t4836, t4838, t4840, t4842, t4843, t4844, t4846, t4847, t4848, t4854)
}
