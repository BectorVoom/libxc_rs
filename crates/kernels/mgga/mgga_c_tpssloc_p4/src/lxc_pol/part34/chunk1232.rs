//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1232/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1232<F: Float>(t101509: F, t105462: F, t105474: F, t1528: F, t17090: F, t17092: F, t259: F, t29056: F, t4268: F, t5558: F, t7823: F, t7830: F, t7842: F, t87779: F, t98921: F, t98923: F, t98927: F) -> F {
    let t108412 = F::new(12.0) * t17092 * t7830 + F::cast_from(0.9869604401089358619e-1_f64) * t105462 + F::cast_from(0.49348022005446793095e-1_f64) * t87779 - F::new(3.0) * t4268 * t29056 - F::new(6.0) * t101509 * t1528 + F::cast_from(0.9869604401089358619e-1_f64) * t105474 + F::cast_from(0.23029076935875170111e0_f64) * t98921 - F::cast_from(0.23029076935875170111e0_f64) * t98923 + F::cast_from(0.49348022005446793095e-1_f64) * t98927 + F::new(3.0) * t5558 * t7823 * t259 - F::new(3.0) * t17090 * t7842;
    t108412
}
