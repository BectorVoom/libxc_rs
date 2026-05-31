//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2389/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2389<F: Float>(t2807: F, t896: F, t13637: F, t41680: F, t41713: F, t47777: F, t48153: F, t48155: F, t48157: F, t48159: F, t48161: F, t48163: F, t48165: F, t48167: F) -> (F, F, F) {
    let t49039 = t2807 * t896;
    let t49040 = t13637 * t49039;
    let t49042 = F::cast_from(0.19931111111111111112e0_f64) * t41680 - F::cast_from(0.59793333333333333333e0_f64) * t41713 + F::cast_from(0.35876000000000000001e1_f64) * t47777 + F::cast_from(0.197176e1_f64) * t48153 + F::cast_from(0.5477111111111111111e0_f64) * t48155 - F::cast_from(0.91285185185185185185e-1_f64) * t48157 - F::cast_from(0.65725333333333333332e0_f64) * t48159 - F::cast_from(0.32862666666666666666e0_f64) * t48161 - F::cast_from(0.32862666666666666667e0_f64) * t48163 + F::cast_from(0.10954222222222222222e0_f64) * t48165 + F::cast_from(0.54771111111111111111e-1_f64) * t48167 - F::cast_from(0.230371875e0_f64) * t49040;
    (t49039, t49040, t49042)
}
