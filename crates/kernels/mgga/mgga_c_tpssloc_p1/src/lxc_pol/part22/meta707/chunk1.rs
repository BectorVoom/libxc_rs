//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2298/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2298<F: Float>(t12725: F, t19451: F, t19456: F, t20100: F, t20109: F, t20136: F, t20717: F, t2314: F, t4028: F, t4034: F, t4072: F, t4077: F, t5107: F, t5460: F, t5493: F, t5494: F, t6287: F, t652: F, t67001: F, t672: F, t7458: F) -> F {
    let t67030 = -F::cast_from(6.0_f64) * t4072 * t6287 * t652 - F::cast_from(6.0_f64) * t5107 * t5493 * t652 - F::cast_from(12.0_f64) * t12725 * t5460 - F::cast_from(6.0_f64) * t12725 * t5494 - F::cast_from(6.0_f64) * t19451 * t4077 - F::cast_from(12.0_f64) * t19456 * t5460 - F::cast_from(6.0_f64) * t20100 * t4028 - F::cast_from(6.0_f64) * t20100 * t7458 - F::cast_from(12.0_f64) * t20109 * t4028 - F::cast_from(12.0_f64) * t20136 * t7458 - F::cast_from(6.0_f64) * t20717 * t2314 - F::cast_from(6.0_f64) * t20717 * t4034 - F::cast_from(2.0_f64) * t67001 * t672;
    t67030
}
