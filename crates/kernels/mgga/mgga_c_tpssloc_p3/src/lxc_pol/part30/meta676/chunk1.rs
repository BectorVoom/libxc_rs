//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2108/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2108<F: Float>(t1863: F, t96469: F, t2240: F, t5399: F, t22544: F, t22549: F, t22551: F, t26009: F, t26013: F, t26016: F, t90114: F, t90192: F, t90248: F, t90251: F, t90330: F, t96443: F, t96454: F, t96458: F, t96462: F, t96466: F) -> F {
    let t96470 = t1863 * t96469;
    let t96473 = t2240 * t5399;
    let t96478 = -F::new(10.0) / F::new(3.0) * t96443 * t22551 - F::new(10.0) / F::new(3.0) * t26016 * t90248 - F::new(10.0) * t90330 * t26009 - F::new(10.0) / F::new(3.0) * t90114 * t26013 - F::new(10.0) * t90192 * t26009 - F::new(10.0) / F::new(3.0) * t22549 * t96454 - F::new(10.0) * t22544 * t96458 - F::new(10.0) / F::new(3.0) * t22549 * t96462 - F::new(5.0) * t22544 * t96466 - F::new(5.0) / F::new(3.0) * t22549 * t96470 - F::new(5.0) / F::new(3.0) * t96473 * t22551 - F::new(10.0) / F::new(3.0) * t26016 * t90251;
    t96478
}
