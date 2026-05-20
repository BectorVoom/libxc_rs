//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2685/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2685<F: Float>(t1307: F, t16018: F, t16084: F, t213: F, t221: F, t3719: F, t40423: F, t40425: F, t40429: F, t40431: F, t5195: F, t54663: F, t54668: F, t54671: F, t54673: F, t54676: F) -> F {
    let t54687 = F::cast_from(0.38888888888888888888e-2_f64) * t40423 - F::cast_from(0.15833333333333333332e-1_f64) * t40425 + F::cast_from(0.83333333333333333332e-3_f64) * t40429 - F::cast_from(0.19999999999999999999e-1_f64) * t54663 + t54668 + F::cast_from(0.46666666666666666664e-1_f64) * t40431 + F::cast_from(0.13999999999999999999e0_f64) * t54671 - F::cast_from(0.34999999999999999998e-1_f64) * t54673 + F::cast_from(0.47499999999999999998e-1_f64) * t54676 + F::cast_from(0.14999999999999999999e-1_f64) * t5195 * t221 * t213 * t16018 * t1307 + F::cast_from(0.14999999999999999999e-1_f64) * t5195 * t221 * t16084 * t3719;
    t54687
}
