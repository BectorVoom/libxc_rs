//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 710/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk710<F: Float>(t225: F, t2666: F, t68: F, t845: F, t2379: F, t2553: F, t824: F, t228: F, t230: F, t822: F, t825: F) -> (F, F, F, F) {
    let t2667 = t2666 * t225;
    let t2671 = t68 * t845;
    let t2672 = t2671 * t2379;
    let t2675 = t824 * t2553;
    let t2678 = -F::new(12.0) * t228 * t2672 + F::new(3.0) * t228 * t2675 - t230 * t2667 + F::new(6.0) * t822 * t825;
    (t2667, t2672, t2675, t2678)
}
