//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2737/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2737<F: Float>(t1390: F, t16497: F, t193: F, t3918: F, t39595: F, t39615: F, t5187: F, t533: F, t56411: F, t56412: F, t56416: F, t56417: F, t56457: F, t56605: F, t56649: F, t57203: F, t57204: F, t57205: F, t57795: F) -> F {
    let t57801 = t39595 + t56411 - t56412 + F::new(12.0) * t3918 * t16497 * t5187 + t56416 - t56417 + t193 * t533 * (t56457 + t56605 + t56649 + t57795) * t1390 - t57203 - t57204 - t57205 + t39615;
    t57801
}
