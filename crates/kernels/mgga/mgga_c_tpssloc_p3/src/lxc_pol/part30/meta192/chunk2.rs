//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 920/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk920<F: Float>(t1241: F, t5088: F, t1238: F, t1252: F, t1761: F, t3487: F, t3593: F, t4941: F, t4943: F, t4945: F, t4947: F, t498: F, t5053: F, t5055: F, t5060: F) -> (F, F) {
    let t5089 = t1241 * t5088;
    let t5091 = F::cast_from(2.0_f64) * t1238 * t5060 - t1238 * t5089 - t1252 * t4945 - t1252 * t5055 - t1761 * t3487 - t1761 * t3593 + t4941 * t498 + t4943 * t498 + t4947 * t498 + t498 * t5053;
    (t5089, t5091)
}
