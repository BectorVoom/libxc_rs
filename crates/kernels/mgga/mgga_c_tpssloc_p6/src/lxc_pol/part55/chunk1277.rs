//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1277/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1277<F: Float>(t32448: F, t4993: F, t10401: F, t117963: F, t117969: F, t117973: F, t119243: F, t1218: F, t1232: F, t125398: F, t125402: F, t125407: F, t125410: F, t125413: F, t1737: F, t32439: F, t32441: F, t3500: F, t4983: F, t5014: F) -> F {
    let t125420 = t32448 * t4993;
    let t125424 = -t3500 * t32439 * t10401 * t119243 * t4983 / F::cast_from(1536.0_f64) + t125398 / F::cast_from(2304.0_f64) - t125402 * t1218 / F::cast_from(288.0_f64) + t125407 * t1232 / F::cast_from(432.0_f64) + t125410 * t1218 / F::cast_from(1536.0_f64) - t125413 * t1232 / F::cast_from(2304.0_f64) + t117973 * t1737 / F::cast_from(1536.0_f64) + t32441 * t5014 / F::cast_from(1536.0_f64) - t125420 / F::cast_from(3456.0_f64) - t117963 / F::cast_from(3456.0_f64) + t117969 / F::cast_from(2304.0_f64);
    t125424
}
