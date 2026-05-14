//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 730/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk730<F: Float>(t24814: F, t24816: F, t1209: F, t24813: F, t1011: F, t475: F, t3611: F, t1193: F, t7372: F, t7378: F, t2147: F, t3590: F, t462: F, t7319: F, t7327: F, t7377: F) -> (F, F, F, F, F) {
    let t24817 = t24814 * t24816;
    let t24820 = t24813 * t1209;
    let t24821 = t1011 * t475;
    let t24822 = t3611 * t24821;
    let t24823 = t24820 * t24822;
    let t24826 = t7372 * t1193;
    let t24827 = t24826 * t7378;
    let t24829 = t2147 * t3590;
    let t24830 = t462 * t24829;
    let t24833 = t7319 * t7327;
    let t24834 = t24833 * t7377;
    (t24817, t24823, t24827, t24830, t24834)
}
