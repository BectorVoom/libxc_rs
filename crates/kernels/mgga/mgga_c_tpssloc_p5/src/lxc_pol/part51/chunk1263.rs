//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1263/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1263<F: Float>(t10109: F, t7841: F, t193: F, t7859: F, t671: F, t7786: F, t12020: F, t7936: F, t214: F, t6624: F, t30657: F, t6547: F) -> (F, F, F, F, F, F) {
    let t101551 = t10109 * t7841;
    let t101840 = t193 * t7859;
    let t102344 = t7786 * t671;
    let t102466 = t12020 * t7936;
    let t112660 = t214 * t6624;
    let t112667 = t6547 * t30657;
    (t101551, t101840, t102344, t102466, t112660, t112667)
}
