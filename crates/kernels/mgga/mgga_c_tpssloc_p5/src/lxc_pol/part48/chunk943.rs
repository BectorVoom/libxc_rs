//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 943/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk943<F: Float>(t652: F, t6534: F, t7156: F, t12823: F, t8533: F, t31772: F, t4034: F, t2018: F, t26161: F, t3698: F, t92169: F, t31338: F, t81651: F, t82074: F) -> (F, F, F, F, F) {
    let t114564 = F::new(4.0) * t652 * t7156 * t6534;
    let t114566 = F::new(2.0) * t12823 * t8533;
    let t114568 = F::new(4.0) * t4034 * t31772;
    let t114573 = F::new(6.0) * t26161 * t92169 * t2018 * t3698;
    let t114592 = t81651 * t82074 * t31338;
    (t114564, t114566, t114568, t114573, t114592)
}
