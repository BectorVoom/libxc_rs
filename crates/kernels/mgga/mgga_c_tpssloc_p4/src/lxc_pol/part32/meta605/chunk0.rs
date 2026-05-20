//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1998/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1998<F: Float>(t22715: F, t6887: F, t6970: F, t12225: F, t22641: F, t22690: F, t6969: F, t268: F, t547: F, t6559: F) -> (F, F, F, F, F) {
    let t81186 = t22715 * t6887;
    let t81187 = t81186 * t6970;
    let t81195 = t22641 * t12225;
    let t81197 = t81195 * t22690 * t6969;
    let t81228 = t6559 * t547 * t268;
    (t81186, t81187, t81195, t81197, t81228)
}
