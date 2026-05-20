//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1709/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1709<F: Float>(t26396: F, t6637: F, t6888: F, t6914: F, t7737: F, t1351: F, t1834: F, t550: F, t6976: F, t1992: F, t3807: F, t5335: F) -> (F, F, F, F, F, F, F) {
    let t26397 = t6637 * t26396;
    let t26398 = t6888 * t26397;
    let t26406 = t6914 * t7737;
    let t26409 = t1834 * t1351;
    let t26410 = t26409 * t550;
    let t26411 = t6976 * t26410;
    let t26412 = t1992 * t26411;
    let t26414 = t5335 * t3807;
    (t26397, t26398, t26406, t26410, t26411, t26412, t26414)
}
