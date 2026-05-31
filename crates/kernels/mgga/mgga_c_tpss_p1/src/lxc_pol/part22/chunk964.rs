//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 964/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk964<F: Float>(t219: F, t3358: F, t1257: F, t73: F, t1219: F, t3357: F, t1270: F, t3387: F, t3202: F, t3205: F, t7651: F, t7653: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10171 = t3358 * t219;
    let t10178 = t1257 * t1257;
    let t10179 = F::cast_from(1.0_f64) / t10178;
    let t10180 = t73 * t10179;
    let t10204 = t1219 * t3357;
    let t10232 = t3387 * t1270;
    let t10236 = t3202 * t3205;
    let t10281 = F::cast_from(4.0_f64) * t7651;
    let t10282 = F::cast_from(12.0_f64) * t7653;
    (t10171, t10178, t10179, t10180, t10204, t10232, t10236, t10281, t10282)
}
