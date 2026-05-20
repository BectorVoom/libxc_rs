//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2074/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2074<F: Float>(t2240: F, t26043: F, t33: F, t45844: F, t6489: F, t111: F, t26097: F, t26351: F, t6883: F, t22751: F, t26186: F, t26190: F) -> (F, F, F, F, F, F) {
    let t90312 = t2240 * t33 * t26043;
    let t90330 = t45844 * t6489;
    let t90400 = t26097 * t111;
    let t90459 = t6883 * t26351;
    let t90460 = F::cast_from(0.38381794893125283518e-1_f64) * t90459;
    let t90468 = t22751 * t26186;
    let t90469 = F::cast_from(0.76763589786250567036e-1_f64) * t90468;
    let t90470 = t22751 * t26190;
    (t90312, t90330, t90400, t90460, t90469, t90470)
}
