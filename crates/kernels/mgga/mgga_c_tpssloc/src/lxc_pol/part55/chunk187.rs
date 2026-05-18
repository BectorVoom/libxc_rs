//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 187/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk187<F: Float>(t33: F, t628: F, t40: F, t73: F, t52: F, t76: F, t607: F) -> (F, F, F, F, F, F) {
    let t629 = t33 * t628;
    let t632 = t40 * t40;
    let t634 = F::new(1.0) / t73 / t632;
    let t636 = t52 * t52;
    let t638 = F::new(1.0) / t76 / t636;
    let t641 = -F::new(4.0) / F::new(3.0) * t634 * t607 + F::new(4.0) / F::new(3.0) * t638 * t607;
    (t629, t632, t634, t636, t638, t641)
}
