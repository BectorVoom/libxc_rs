//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1626/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1626<F: Float>(t23228: F, t6554: F, t23171: F, t23168: F, t6556: F, t6547: F, t6573: F, t214: F, t852: F) -> (F, F, F, F, F, F, F) {
    let t23229 = t23228 * t6554;
    let t23230 = t23171 * t23229;
    let t23232 = t23168 * t6556;
    let t23233 = F::cast_from(0.76763589786250567036e-1_f64) * t23232;
    let t23235 = t6547 * t6573;
    let t23236 = F::cast_from(0.38381794893125283518e-1_f64) * t23235;
    let t23237 = t214 * t852;
    (t23229, t23230, t23232, t23233, t23235, t23236, t23237)
}
