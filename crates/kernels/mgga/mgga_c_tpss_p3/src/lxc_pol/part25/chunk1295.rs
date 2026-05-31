//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1295/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1295<F: Float>(t18670: F, t19408: F, t1791: F, t65157: F, t65165: F, t19342: F, t62348: F, t19349: F, t62342: F, t65208: F, t1675: F, t18645: F, t6090: F) -> (F, F, F, F, F, F, F) {
    let t67337 = F::cast_from(80.0_f64) / F::cast_from(9.0_f64) * t18670 * t19408;
    let t67349 = t1791 * t65157;
    let t67352 = t1791 * t65165;
    let t67358 = F::cast_from(160.0_f64) / F::cast_from(3.0_f64) * t62348 * t19342;
    let t67369 = F::cast_from(160.0_f64) / F::cast_from(9.0_f64) * t19349 * t62342;
    let t67378 = t1791 * t65208;
    let t67385 = t1675 * t18645 * t6090;
    (t67337, t67349, t67352, t67358, t67369, t67378, t67385)
}
