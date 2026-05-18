//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1243/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1243<F: Float>(t23030: F, t30660: F, t23204: F, t30656: F, t6562: F, t30624: F, t81591: F, t30635: F, t6579: F, t23185: F, t30634: F, t82074: F) -> (F, F, F, F, F) {
    let t112676 = F::new(0.52089578783527170489e-1) * t23030 * t30660;
    let t112678 = t6562 * t23204 * t30656;
    let t112680 = t81591 * t30624;
    let t112686 = t6579 * t30635;
    let t112702 = t23185 * t82074 * t30634;
    (t112676, t112678, t112680, t112686, t112702)
}
