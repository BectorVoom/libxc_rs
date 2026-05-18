//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1244/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1244<F: Float>(t22690: F, t6969: F, t81195: F, t1338: F, t22870: F, t2006: F, t3850: F, t22881: F, t3719: F, t6637: F, t6888: F, t12012: F, t6968: F) -> (F, F, F, F, F) {
    let t81197 = t81195 * t22690 * t6969;
    let t81199 = t1338 * t22870;
    let t81203 = t2006 * t3850;
    let t81209 = t6888 * t6637 * t22881 * t3719;
    let t81213 = t6888 * t6637 * t6968 * t12012;
    (t81197, t81199, t81203, t81209, t81213)
}
