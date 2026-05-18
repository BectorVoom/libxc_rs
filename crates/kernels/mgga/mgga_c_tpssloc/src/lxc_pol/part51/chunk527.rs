//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 527/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk527<F: Float>(t515: F, t518: F, t215: F, t2559: F, t535: F, t1314: F, t782: F, t1317: F, t2566: F, t795: F, t154: F, t557: F) -> (F, F, F, F, F, F, F) {
    let t3704 = F::new(1.0) / t515;
    let t3711 = F::new(1.0) / t518;
    let t3725 = F::new(0.64814814814814814813e-2) * t2559 * t535 * t215;
    let t3726 = t782 * t1314;
    let t3727 = t3726 * t1317;
    let t3731 = F::new(0.26388888888888888888e-2) * t2566 * t535 * t795;
    let t3732 = t154 * t557;
    (t3704, t3711, t3725, t3726, t3727, t3731, t3732)
}
