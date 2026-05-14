//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 938/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk938<F: Float>(t28099: F, t550: F, t1339: F, t22827: F, t22833: F, t6396: F, t1842: F, t26337: F, t22635: F, t22633: F, t1825: F, t26421: F, t6976: F, t19743: F, t3792: F, t22897: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28100 = t28099 * t550;
    let t28101 = t1339 * t28100;
    let t28102 = t22827 * t28101;
    let t28104 = t22833 * t6396;
    let t28116 = t26337 * t1842;
    let t28117 = t22635 * t28116;
    let t28118 = t22633 * t28117;
    let t28130 = t26421 * t1825;
    let t28131 = t6976 * t28130;
    let t28132 = t22633 * t28131;
    let t28134 = t19743 * t3792;
    let t28135 = t22897 * t28134;
    (t28100, t28101, t28102, t28104, t28116, t28117, t28118, t28130, t28131, t28132, t28134, t28135)
}
