//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1108/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1108<F: Float>(t1222: F, t8049: F, t5017: F, t7337: F, t1207: F, t1218: F, t2136: F, t24675: F, t24681: F, t24690: F, t24704: F, t27578: F, t27580: F, t27586: F, t27589: F, t488: F, t4974: F, t5014: F, t5030: F, t7339: F, t7345: F) -> F {
    let t27592 = t8049 * t1222;
    let t27598 = t7337 * t5017;
    let t27599 = t1207 * t27598;
    let t27602 = t24675 / F::new(2304.0) - t24681 + t27578 / F::new(2304.0) + F::cast_from(0.80745512188280781712e-3_f64) * t27580 * t2136 - t7345 * t4974 / F::new(1152.0) - t24690 / F::new(864.0) - t24704 + t27586 * t488 / F::new(1536.0) - t27589 * t488 / F::new(288.0) - t27592 / F::new(432.0) - t7345 * t5030 / F::new(2304.0) + t7339 * t5014 / F::new(1536.0) - t27599 * t1218 / F::new(288.0);
    t27602
}
