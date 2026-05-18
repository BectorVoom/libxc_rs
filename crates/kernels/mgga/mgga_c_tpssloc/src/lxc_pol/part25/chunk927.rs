//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 927/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk927<F: Float>(t11599: F, t11601: F, t11608: F, t11613: F, t11919: F, t11923: F, t11925: F, t11928: F, t11931: F, t11935: F, t1238: F, t1252: F, t3487: F, t3593: F, t3600: F, t3631: F, t498: F) -> F {
    let t11940 = t11599 * t498 + F::new(3.0) * t11601 * t498 - F::new(6.0) * t11608 * t1238 - F::new(6.0) * t11613 * t1252 - t11919 * t1238 + t11923 * t498 - F::new(3.0) * t11925 * t1252 - F::new(3.0) * t11928 * t1252 + F::new(3.0) * t11931 * t498 + F::new(6.0) * t11935 * t1238 + F::new(6.0) * t3487 * t3600 - F::new(3.0) * t3487 * t3631 + F::new(6.0) * t3593 * t3600 - F::new(3.0) * t3593 * t3631;
    t11940
}
