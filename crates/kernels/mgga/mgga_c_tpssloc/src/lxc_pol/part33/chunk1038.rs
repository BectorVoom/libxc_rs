//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1038/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1038<F: Float>(t1920: F, t25529: F, t23384: F, t7604: F, t4640: F, t6754: F, t1611: F, t6764: F, t4603: F, t6717: F, t4571: F, t6765: F, t4630: F, t6755: F, t1036: F, t7586: F) -> (F, F, F, F, F, F, F, F) {
    let t25530 = t1920 * t25529;
    let t25563 = t23384 * t7604;
    let t25577 = t4640 * t6754;
    let t25580 = t1611 * t6764;
    let t25598 = t6717 * t4603;
    let t25616 = t6765 * t4571;
    let t25618 = t6755 * t4630;
    let t25625 = t7586 * t1036;
    (t25530, t25563, t25577, t25580, t25598, t25616, t25618, t25625)
}
