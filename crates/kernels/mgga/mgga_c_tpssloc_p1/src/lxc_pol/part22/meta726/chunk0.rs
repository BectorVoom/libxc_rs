//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2379/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2379<F: Float>(t48140: F, t48143: F, t68513: F, t42444: F, t20234: F, t41687: F, t607: F, t10304: F, t136: F, t17151: F, t3966: F, t41880: F, t68477: F) -> (F, F, F, F, F, F, F) {
    let t68515 = t48140 * t48143 * t68513;
    let t68518 = t48140 * t42444 * t68513;
    let t68521 = t41687 * t20234 * t607;
    let t68523 = t136 * t10304 * t68521;
    let t68525 = t17151 * t3966;
    let t68527 = t136 * t10304 * t68525;
    let t68530 = t136 * t41880 * t68477;
    (t68515, t68518, t68521, t68523, t68525, t68527, t68530)
}
