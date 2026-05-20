//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1002/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1002<F: Float>(t11632: F, t11637: F, t491: F, t1246: F, t1222: F, t3567: F, t1203: F, t3540: F, t2393: F, t374: F, t486: F, t485: F) -> (F, F, F, F, F, F, F) {
    let t11638 = t11632 + t11637;
    let t11639 = t491 * t11638;
    let t11640 = t11639 * t1246;
    let t11642 = t3567 * t1222;
    let t11644 = t1203 * t3540;
    let t11647 = t374 * t2393 * t486;
    let t11649 = t485 * t11647 / F::new(10368.0);
    (t11638, t11639, t11640, t11642, t11644, t11647, t11649)
}
