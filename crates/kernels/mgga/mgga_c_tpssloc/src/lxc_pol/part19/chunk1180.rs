//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1180/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1180<F: Float>(t41678: F, t41680: F, t41682: F, t41684: F, t41690: F, t41695: F, t41699: F, t41703: F, t41707: F, t41711: F, t41713: F, t41717: F, t324: F, t41749: F, t10603: F, t2932: F) -> (F, F) {
    let t41762 = -0.49444444444444444444e-1 * t41678 + 0.24722222222222222222e-1 * t41680 + 0.74166666666666666668e-1 * t41682 + 0.38456790123456790123e-1 * t41684 + 0.12361111111111111111e0 * t41690 - 0.61805555555555555555e-1 * t41695 - 0.22249999999999999999e0 * t41699 - 0.18541666666666666666e-1 * t41703 - 0.24722222222222222222e-1 * t41707 + 0.2225e0 * t41711 - 0.74166666666666666668e-1 * t41713 - 0.33375e0 * t41717;
    let t41764 = (t41749 + t41762) * t324;
    let t41769 = t10603 * t2932;
    (t41764, t41769)
}
