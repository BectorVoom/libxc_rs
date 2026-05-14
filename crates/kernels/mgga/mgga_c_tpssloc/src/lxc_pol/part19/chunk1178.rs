//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1178/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1178<F: Float>(t41678: F, t41680: F, t41682: F, t41684: F, t41690: F, t41695: F, t41699: F, t41703: F, t41707: F, t41711: F, t41713: F, t41717: F, t291: F, t41677: F, t10603: F, t2929: F, t4497: F, t959: F) -> (F, F) {
    let t41719 = -0.94977777777777777776e-1 * t41678 + 0.47488888888888888888e-1 * t41680 + 0.14246666666666666667e0 * t41682 + 0.73871604938271604937e-1 * t41684 + 0.23744444444444444444e0 * t41690 - 0.11872222222222222222e0 * t41695 - 0.42739999999999999999e0 * t41699 - 0.35616666666666666666e-1 * t41703 - 0.47488888888888888888e-1 * t41707 + 0.4274e0 * t41711 - 0.14246666666666666667e0 * t41713 - 0.6411e0 * t41717;
    let t41722 = 0.621814e-1 * (t41677 + t41719) * t291;
    let t41726 = 0.69263436422725855036e2 * t959 * t2929 * t10603 * t4497;
    (t41722, t41726)
}
