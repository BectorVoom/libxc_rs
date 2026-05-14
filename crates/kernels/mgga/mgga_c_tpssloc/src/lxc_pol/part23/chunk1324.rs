//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1324/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1324<F: Float>(t6052: F, t11310: F, t11350: F, t1137: F, t11420: F, t15136: F, t15146: F, t1682: F, t18650: F, t21836: F, t21907: F, t21952: F, t3332: F, t3357: F, t3359: F, t3403: F, t436: F, t51680: F, t6037: F, t6069: F, t63454: F, t71729: F, t78287: F, t78359: F, t78361: F, t78364: F, t78367: F, t78370: F, t78373: F, t78859: F, t78961: F, t78973: F) -> (F,) {
    let t78988 = t6052 * t6052;
    let t79002 = 36.0 * t3357 * t6037 * t6052 - 0.14035736694323150897e2 * t15136 * t21836 - 0.310907e-1 * (t78961 + t78973) * t436 + t78359 - t78361 + t78364 + t78367 - t78370 - t78373 + 0.12865583598954028054e3 * t3357 * t71729 * t1682 + 0.12414243100625616072e5 * t11350 * t18650 * t6052 + 24.0 * t15146 * t21952 - 24.0 * t11420 * t78859 * t1137 - 6.0 * t3332 * t78988 * t1137 + 0.96491876992155210402e2 * t3357 * t78988 * t3359 - 0.70178683471615754484e1 * t63454 * t6069 - 0.4155806185363551302e3 * t51680 * t21907 + 0.6233709278045326953e3 * t11310 * t78287 * t3403;
    (t79002,)
}
