//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2703/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2703<F: Float>(t1410: F, t1434: F, t19335: F, t19338: F, t19343: F, t19346: F, t19349: F, t19404: F, t20227: F, t3961: F, t3967: F, t4018: F, t5400: F, t5403: F, t5427: F, t642: F, t80: F) -> F {
    let t75419 = -t19335 * t1434 / F::new(4.0) - t19338 * t1434 / F::new(4.0) - t5400 * t4018 / F::new(4.0) - t3961 * t5427 * t80 / F::new(4.0) - t3967 * t5427 * t80 / F::new(4.0) - t1410 * t19404 * t80 / F::new(4.0) - t20227 * t642 / F::new(4.0) - t19343 * t1434 / F::new(2.0) - t19346 * t1434 / F::new(2.0) - t19349 * t1434 / F::new(2.0) - t5403 * t4018 / F::new(2.0);
    t75419
}
