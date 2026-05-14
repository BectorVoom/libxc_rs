//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 625/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk625<F: Float>(t1358: F, t2003: F, t552: F, t59: F, t240: F, t1336: F) -> (F, F, F, F) {
    let t6948 = t2003 * t1358;
    let t6950 = t552 * t59;
    let t6951 = t6950 * t240;
    let t6952 = t1336 * t6951;
    (t6948, t6950, t6951, t6952)
}
