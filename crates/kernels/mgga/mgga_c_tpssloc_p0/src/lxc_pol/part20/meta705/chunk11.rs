//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2689/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2689<F: Float>(t54647: F, t54658: F, t54687: F, t54736: F, t225: F, t1336: F, t242: F, t40042: F, t12177: F, t40046: F, t16391: F, t16398: F) -> (F, F, F, F, F) {
    let t54738 = t54647 + t54658 + t54687 + t54736;
    let t54739 = t54738 * t225;
    let t54744 = t1336 * t40042 * t242;
    let t54745 = t40046 * t12177;
    let t54750 = t16398 * t16391;
    (t54738, t54739, t54744, t54745, t54750)
}
