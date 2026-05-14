//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 180/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk180<F: Float>(t761: F, t763: F, t201: F, t262: F, t73: F, t76: F, t583: F, t60: F, t59: F) -> (F, F, F, F, F, F) {
    let t765 = 0.5848223622634646207e0 * t761 * t763;
    let t766 = t201 * t262;
    let t767 = 1.0 / t73;
    let t771 = 1.0 / t76;
    let t781 = 1.0 / t60 / t583;
    let t782 = t59 * t781;
    (t765, t766, t767, t771, t781, t782)
}
