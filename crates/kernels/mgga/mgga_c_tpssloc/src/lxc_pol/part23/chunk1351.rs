//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1351/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1351<F: Float>(t109: F, t79816: F, t5493: F, t5449: F, t5456: F, t53777: F, t53779: F, t56099: F, t56104: F, t73967: F, t53798: F, t1799: F, t19596: F, t20067: F, t20675: F, t28830: F, t3918: F, t39249: F, t39256: F, t39261: F, t5160: F, t5161: F, t6347: F, t74068: F, t75240: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t110 = 1.0 < t109;
    let t79817 = piecewise3(t110, 0.0, t79816);
    let t79825 = t5493 * t5493;
    let t79829 = t5449 * t5456;
    let t79834 = 0.86748650402413918736e-1 * t53777;
    let t79835 = 0.1301229756036208781e0 * t53779;
    let t79836 = 0.10389515463408878255e3 * t56099;
    let t79837 = 0.35089341735807877242e1 * t56104;
    let t79853 = 0.73245789224026180216e-3 * t73967;
    let t79854 = 0.14035736694323150897e2 * t53798;
    let t79855 = 12.0 * t1799 * t3918 * t74068 + 24.0 * t1799 * t3918 * t75240 - 36.0 * t19596 * t28830 * t3918 + 18.0 * t20067 * t3918 * t6347 - 4.0 * t20675 * t5160 * t5161 - t39249 - t39256 - t39261 - t79834 - t79835 - t79836 - t79837 - t79853 - t79854;
    (t79817, t79825, t79829, t79834, t79835, t79836, t79837, t79853, t79854, t79855)
}
