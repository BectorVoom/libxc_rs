//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2514/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2514<F: Float>(t50834: F, t51550: F, t63291: F, t63306: F, t63308: F, t63332: F, t63334: F, t63336: F, t71124: F, t71130: F, t71135: F, t71140: F, t71142: F, t71144: F, t71146: F, t71150: F, t71152: F, t71154: F, t71156: F, t71160: F) -> F {
    let t71162 = -F::cast_from(0.35616666666666666666e-1_f64) * t63291 + F::cast_from(0.11872222222222222222e-1_f64) * t63306 - F::cast_from(0.19787037037037037037e-1_f64) * t63308 + t51550 - F::cast_from(0.55403703703703703703e-1_f64) * t50834 + F::cast_from(0.5936111111111111111e-1_f64) * t71124 - F::cast_from(0.15829629629629629629e-1_f64) * t63332 + F::cast_from(0.23744444444444444444e-1_f64) * t63334 - F::cast_from(0.17808333333333333333e-1_f64) * t63336 - F::cast_from(0.21369999999999999999e0_f64) * t71130 + F::cast_from(0.23744444444444444444e0_f64) * t71135 - F::cast_from(0.11872222222222222222e-1_f64) * t71140 + F::cast_from(0.11872222222222222222e-1_f64) * t71142 - F::cast_from(0.35616666666666666667e-1_f64) * t71144 - F::cast_from(0.65956790123456790123e-2_f64) * t71146 + F::cast_from(0.17808333333333333333e-1_f64) * t71150 - F::cast_from(0.35616666666666666667e-1_f64) * t71152 - F::cast_from(0.5936111111111111111e-2_f64) * t71154 + F::cast_from(0.23744444444444444444e-1_f64) * t71156 + F::cast_from(0.59361111111111111111e-1_f64) * t71160;
    t71162
}
