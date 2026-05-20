//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta532 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1786;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1787;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta532<F: Float>(t22734: F, t81159: F, t22899: F, t6914: F, t22715: F, t6887: F, t6970: F, t22751: F, t22883: F, t12225: F, t22641: F, t22690: F, t6969: F, t22886: F, t22892: F, t22893: F, t22887: F, t268: F, t547: F, t6559: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t81160, t81184, t81186, t81187, t81189, t81195, t81197) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1786::<F>(t22734, t81159, t22899, t6914, t22715, t6887, t6970, t22751, t22883, t12225, t22641, t22690, t6969);
        let (t81216, t81218, t81228) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1787::<F>(t22886, t22892, t22893, t22751, t22887, t268, t547, t6559);
    (t81160, t81184, t81186, t81187, t81189, t81195, t81197, t81216, t81218, t81228)
}
