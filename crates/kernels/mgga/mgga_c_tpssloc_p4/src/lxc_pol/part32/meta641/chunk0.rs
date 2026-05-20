//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2059/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2059<F: Float>(t2240: F, t3967: F, t12571: F, t608: F, t645: F, t7445: F, t26351: F, t6883: F, t22751: F, t26186: F, t26190: F, t26356: F, t6914: F) -> (F, F, F, F, F, F, F) {
    let t90104 = t2240 * t3967;
    let t90114 = t12571 * t608;
    let t90247 = t7445 * t645;
    let t90459 = t6883 * t26351;
    let t90460 = F::cast_from(0.38381794893125283518e-1_f64) * t90459;
    let t90468 = t22751 * t26186;
    let t90469 = F::cast_from(0.76763589786250567036e-1_f64) * t90468;
    let t90470 = t22751 * t26190;
    let t90471 = F::cast_from(0.76763589786250567036e-1_f64) * t90470;
    let t90472 = t6914 * t26356;
    (t90104, t90114, t90247, t90460, t90469, t90471, t90472)
}
