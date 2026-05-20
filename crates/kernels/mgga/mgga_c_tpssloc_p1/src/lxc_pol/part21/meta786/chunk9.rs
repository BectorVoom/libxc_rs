//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2735/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2735<F: Float>(t12267: F, t1336: F, t1352: F, t1380: F, t16047: F, t16048: F, t16123: F, t16433: F, t1840: F, t19660: F, t19733: F, t19743: F, t19744: F, t19756: F, t3777: F, t3793: F, t3856: F, t5234: F, t5250: F, t5334: F, t5344: F, t57300: F, t57607: F, t57704: F, t6451: F) -> F {
    let t57760 = -t1336 * t1380 * t57300 - F::new(2.0) * t1352 * t5344 * t57704 - F::new(6.0) * t16047 * t16048 * t19660 - F::new(12.0) * t16047 * t19744 * t57607 + F::new(6.0) * t19660 * t3793 * t5334 - t19660 * t3856 * t5344 + F::new(14.0) * t19743 * t3793 * t5334 - t19743 * t3856 * t5344 + F::new(12.0) * t5250 * t5334 * t57607 - F::new(2.0) * t12267 * t6451 + F::new(2.0) * t16123 * t1840 - F::new(2.0) * t16433 * t5234 - F::new(2.0) * t19733 * t3777 - F::new(4.0) * t19756 * t3777;
    t57760
}
