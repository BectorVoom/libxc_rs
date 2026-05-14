//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 738/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk738<F: Float>(t23917: F, t510: F, t1266: F, t7056: F, t671: F, t7156: F, t111: F, t7039: F) -> (F, F, F, F) {
    let t23918 = t510 * t23917;
    let t23929 = t1266 * t7056;
    let t23933 = t7156 * t671;
    let t23938 = t7039 * t111;
    (t23918, t23929, t23933, t23938)
}
