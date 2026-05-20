//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1319/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1319<F: Float>(t10472: F, t10475: F, t42559: F, t3128: F, t10903: F, t10948: F, t10890: F, t10898: F, t3103: F, t1000: F, t10390: F, t10405: F, t10410: F, t10415: F, t10485: F, t10860: F, t10879: F, t10919: F, t3043: F, t3109: F, t3117: F, t3123: F, t3134: F, t42541: F, t42546: F, t42552: F, t42554: F, t42557: F) -> F {
    let t42561 = t10472 * t10475 * t42559;
    let t42565 = t10472 * t3128 * t42559;
    let t42570 = t10948 * t10903;
    let t42573 = t10948 * t10890;
    let t42578 = t10898 * t3103;
    let t42580 = t42541 * t10405 / F::new(192.0) + F::new(5.0) / F::new(1152.0) * t10390 * t10410 - t42546 * t10415 / F::new(384.0) + F::new(5.0) / F::new(1152.0) * t3117 * t10919 + F::new(5.0) / F::new(972.0) * t42552 - F::new(154.0) / F::new(243.0) * t42554 * t1000 + F::new(11.0) / F::new(81.0) * t42557 - t42561 * t10485 / F::new(24.0) + t42565 * t10879 / F::new(24.0) - t10898 * t3123 / F::new(48.0) - t42570 * t3134 / F::new(24.0) + t42573 * t3043 / F::new(48.0) - t3109 * t10860 / F::new(144.0) - t42578 / F::new(36.0);
    t42580
}
