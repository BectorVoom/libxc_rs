//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1141/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1141<F: Float>(t120714: F, t120716: F, t120719: F, t120721: F, t120723: F, t120728: F, t120730: F, t120732: F, t120735: F, t120738: F, t120740: F, t120742: F, t120744: F, t120747: F, t120749: F, t120751: F, t120753: F, t31062: F, t31224: F, t4028: F, t4077: F) -> (F,) {
    let t120755 = -2.0 * t31062 * t4028 - 2.0 * t31224 * t4077 - 4.0 * t120714 - 4.0 * t120716 - t120719 - t120721 - 4.0 * t120723 - t120728 - t120730 - 4.0 * t120732 - t120735 - 2.0 * t120738 - 4.0 * t120740 - 4.0 * t120742 - 4.0 * t120744 - 4.0 * t120747 - 4.0 * t120749 - 4.0 * t120751 - 4.0 * t120753;
    (t120755,)
}
