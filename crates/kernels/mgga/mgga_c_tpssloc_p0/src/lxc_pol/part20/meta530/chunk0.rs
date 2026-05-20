//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2065/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2065<F: Float>(t1307: F, t3850: F, t12392: F, t3799: F, t39037: F, t522: F, t2221: F, t3826: F, t12132: F, t592: F, t3696: F, t1336: F, t1339: F, t2691: F) -> (F, F, F, F, F, F, F) {
    let t40197 = t1307 * t3850;
    let t40206 = t3799 * t12392;
    let t40224 = F::new(840.0) * t39037 * t522;
    let t40225 = t2221 * t3826;
    let t40230 = F::new(16.0) * t592 * t12132;
    let t40231 = t2221 * t3696;
    let t40281 = t1336 * t1339 * t2691;
    (t40197, t40206, t40224, t40225, t40230, t40231, t40281)
}
