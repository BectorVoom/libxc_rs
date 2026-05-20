//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1786/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1786<F: Float>(t22734: F, t81159: F, t22899: F, t6914: F, t22715: F, t6887: F, t6970: F, t22751: F, t22883: F, t12225: F, t22641: F, t22690: F, t6969: F) -> (F, F, F, F, F, F, F) {
    let t81160 = t81159 * t22734;
    let t81184 = t6914 * t22899;
    let t81186 = t22715 * t6887;
    let t81187 = t81186 * t6970;
    let t81189 = t22751 * t22883;
    let t81195 = t22641 * t12225;
    let t81197 = t81195 * t22690 * t6969;
    (t81160, t81184, t81186, t81187, t81189, t81195, t81197)
}
