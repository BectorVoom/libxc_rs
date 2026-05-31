//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1336/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1336<F: Float>(t6308: F, t645: F, t4637: F, t5798: F, t19349: F, t20264: F, t62259: F, t62262: F, t65169: F, t65172: F, t65175: F, t67331: F, t67333: F, t67335: F, t67337: F, t67349: F, t67358: F, t67369: F) -> (F, F, F) {
    let t71344 = t6308 * t645;
    let t71374 = t5798 * t4637;
    let t71386 = t67331 + t67333 + t67335 + t67337 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t62259 + F::cast_from(88.0_f64) / F::cast_from(27.0_f64) * t62262 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t65169 * t20264 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t65172 * t20264 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t65175 * t20264 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t19349 * t67349 - t67358 - t67369;
    (t71344, t71374, t71386)
}
