//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 467/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk467<F: Float>(t215: F, t2559: F, t535: F, t1314: F, t782: F, t1317: F, t2566: F, t795: F, t154: F, t557: F, t205: F, t1307: F) -> (F, F, F, F, F, F) {
    let t3725 = 0.64814814814814814813e-2 * t2559 * t535 * t215;
    let t3726 = t782 * t1314;
    let t3727 = t3726 * t1317;
    let t3731 = 0.26388888888888888888e-2 * t2566 * t535 * t795;
    let t3732 = t154 * t557;
    let t3733 = t205 * t3732;
    let t3734 = t1307 * t1307;
    (t3725, t3726, t3727, t3731, t3733, t3734)
}
