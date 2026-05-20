//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2240/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2240<F: Float>(t1888: F, t232: F, t5631: F, t6646: F, t828: F, t25319: F, t4119: F, t6552: F, t6637: F, t16935: F, t17034: F, t25261: F, t25281: F, t4162: F, t4281: F, t5575: F, t6660: F, t7535: F, t81689: F, t81717: F, t82011: F, t87604: F, t87613: F, t87619: F, t87635: F, t87669: F, t87680: F, t92781: F, t92794: F) -> F {
    let t98571 = t1888 * t6646 * t5631 * t828 * t232;
    let t98575 = t6552 * t6637 * t25319 * t4119;
    let t98587 = t87604 - F::cast_from(0.82246703342411321825e-2_f64) * t98571 - t81689 - F::cast_from(0.3289868133696452873e-1_f64) * t98575 - t87613 + t87619 - F::cast_from(0.25587863262083522345e0_f64) * t87635 + F::new(4.0) * t4281 * t25261 * t16935 - t92781 + t81717 + F::new(4.0) * t17034 * t25281 - t92794 + t87669 + t87680 + t5575 * t6660 - F::cast_from(0.63969658155208805863e-1_f64) * t82011 + F::new(2.0) * t4162 * t7535;
    t98587
}
