//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 216/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk216<F: Float>(t1206: F, t1243: F, t357: F, t475: F, t500: F, t111: F, t88: F) -> (F, F, F, F) {
    let t1244 = t1206 * t1243;
    let t1246 = t357 * t475;
    let t1256 = 1.0 / t500;
    let t1268 = t88 * t111;
    (t1244, t1246, t1256, t1268)
}
