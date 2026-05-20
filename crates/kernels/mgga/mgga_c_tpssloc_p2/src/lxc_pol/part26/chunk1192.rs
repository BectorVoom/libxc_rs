//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1192/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1192<F: Float>(t254: F, t492: F, t11605: F, t2154: F, t460: F, t491: F, t7286: F, t2113: F, t671: F, t1902: F, t828: F, t3701: F, t6995: F) -> (F, F, F, F, F, F) {
    let t27784 = t492 * t254;
    let t27785 = t11605 * t2154;
    let t27798 = t460 * t491;
    let t27799 = t27798 * t7286;
    let t27888 = t2113 * t671;
    let t30684 = t1902 * t828;
    let t31035 = t3701 * t6995;
    (t27784, t27785, t27799, t27888, t30684, t31035)
}
