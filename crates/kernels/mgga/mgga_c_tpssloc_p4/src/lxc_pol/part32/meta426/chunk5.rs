//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1650/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1650<F: Float>(t1268: F, t12725: F, t1458: F, t19450: F, t19451: F, t19456: F, t19461: F, t19534: F, t2314: F, t4028: F, t4072: F, t5113: F, t5493: F, t671: F, t7676: F) -> F {
    let t19537 = F::new(2.0) * t1268 * t19534 + F::new(4.0) * t12725 * t1458 + F::new(4.0) * t1458 * t19456 + F::new(2.0) * t19451 * t671 + F::new(2.0) * t2314 * t5493 + F::new(4.0) * t4028 * t4072 + F::new(4.0) * t4072 * t7676 + F::new(2.0) * t5113 * t5493 + t19450 + F::new(2.0) * t19461;
    t19537
}
