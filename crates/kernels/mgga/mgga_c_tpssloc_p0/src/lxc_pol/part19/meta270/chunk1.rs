//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1027/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1027<F: Float>(t11599: F, t11601: F, t11608: F, t11613: F, t11919: F, t11923: F, t11925: F, t11928: F, t11931: F, t11935: F, t1238: F, t1252: F, t3487: F, t3593: F, t3600: F, t3631: F, t498: F) -> F {
    let t11940 = t11599 * t498 + F::cast_from(3.0_f64) * t11601 * t498 - F::cast_from(6.0_f64) * t11608 * t1238 - F::cast_from(6.0_f64) * t11613 * t1252 - t11919 * t1238 + t11923 * t498 - F::cast_from(3.0_f64) * t11925 * t1252 - F::cast_from(3.0_f64) * t11928 * t1252 + F::cast_from(3.0_f64) * t11931 * t498 + F::cast_from(6.0_f64) * t11935 * t1238 + F::cast_from(6.0_f64) * t3487 * t3600 - F::cast_from(3.0_f64) * t3487 * t3631 + F::cast_from(6.0_f64) * t3593 * t3600 - F::cast_from(3.0_f64) * t3593 * t3631;
    t11940
}
