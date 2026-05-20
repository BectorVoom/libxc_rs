//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1920/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1920<F: Float>(t1992: F, t54840: F, t550: F, t6976: F, t54883: F, t22633: F, t22897: F, t26421: F, t3793: F, t16041: F, t22704: F, t5336: F, t80798: F) -> (F, F, F, F, F) {
    let t90883 = t1992 * t6976 * t54840 * t550;
    let t90887 = t1992 * t6976 * t54883 * t550;
    let t90892 = t22633 * t22897 * t26421 * t3793;
    let t90895 = t1992 * t22897 * t16041;
    let t90898 = t22704 * t80798 * t5336;
    (t90883, t90887, t90892, t90895, t90898)
}
