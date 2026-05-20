//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 666/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk666<F: Float>(t1213: F, t1227: F, t1737: F, t1748: F, t3506: F, t3515: F, t3542: F, t3547: F, t467: F, t5005: F, t5019: F, t5024: F, t5036: F, t5041: F, t6109: F, t6203: F, t6207: F, t6211: F, t6221: F, t6227: F, t6232: F) -> F {
    let t6237 = -t5005 * t1748 / F::new(2304.0) - t5019 * t1737 / F::new(288.0) + F::new(5.0) / F::new(13824.0) * t1227 * t6203 - t1227 * t6207 / F::new(4608.0) - t1227 * t6211 / F::new(2304.0) - t5036 / F::new(54.0) + F::new(11.0) / F::new(108.0) * t6109 * t467 - t5041 / F::new(432.0) - t3542 + t1213 * t6221 / F::new(3072.0) + t3506 * t6227 / F::new(1536.0) - t3515 * t6232 / F::new(3072.0) + t5024 * t1748 / F::new(432.0) - t3547;
    t6237
}
