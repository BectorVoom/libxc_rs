//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1169/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1169<F: Float>(t22899: F, t6914: F, t22715: F, t6887: F, t6970: F, t22751: F, t22883: F, t22685: F, t22881: F, t3734: F, t6637: F, t12225: F, t22641: F, t22690: F, t6969: F, t1338: F, t22870: F) -> (F, F, F, F, F, F, F, F) {
    let t81184 = t6914 * t22899;
    let t81186 = t22715 * t6887;
    let t81187 = t81186 * t6970;
    let t81189 = t22751 * t22883;
    let t81193 = t22685 * t6637 * t22881 * t3734;
    let t81195 = t22641 * t12225;
    let t81197 = t81195 * t22690 * t6969;
    let t81199 = t1338 * t22870;
    (t81184, t81186, t81187, t81189, t81193, t81195, t81197, t81199)
}
