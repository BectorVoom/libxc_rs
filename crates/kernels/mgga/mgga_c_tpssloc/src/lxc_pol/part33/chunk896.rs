//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 896/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk896<F: Float>(t20818: F, t20820: F, t20822: F, t20823: F, t20824: F, t20827: F, t20829: F, t20830: F, t20831: F, t9853: F, t9859: F, t9894: F, t9907: F, t9921: F, t20752: F, t20772: F, t21073: F) -> (F,) {
    let t21074 = t20818 - t9894 + t20820 + t20822 + t9907 - t20823 + t20824 + t20827 + t9853 + t20829 - t9921 + t20830 - t20831 + t9859;
    let t21076 = t20752 + t20772 + t21073 + t21074;
    (t21076,)
}
