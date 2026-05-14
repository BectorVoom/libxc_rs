//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 642/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk642<F: Float>(t22817: F, t22818: F, t61: F, t9222: F, t1995: F, t133: F, t6933: F, t6604: F, t6925: F, t16312: F, t550: F, t1339: F, t242: F, t6943: F, t1336: F, t3809: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t22819 = t22817 * t22818;
    let t22820 = 0.16821981705891829522e-4 * t22819;
    let t22822 = 1.0 / t61 / t9222;
    let t22823 = t22822 * t1995;
    let t22824 = t22823 * t133;
    let t22825 = t22824 * t6933;
    let t22826 = 0.52708876011794399171e-3 * t22825;
    let t22827 = t6925 * t6604;
    let t22828 = t16312 * t550;
    let t22829 = t1339 * t22828;
    let t22830 = t22827 * t22829;
    let t22832 = t6943 * t242;
    let t22833 = t1336 * t22832;
    let t22834 = t22833 * t3809;
    (t22819, t22820, t22822, t22824, t22825, t22826, t22827, t22828, t22830, t22834)
}
