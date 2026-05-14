//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 901/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk901<F: Float>(t10164: F, t525: F, t219: F, t3358: F, t1257: F, t73: F, t1219: F, t3357: F, t1270: F, t3387: F, t3202: F, t3205: F, t7651: F, t7653: F, t7660: F, t7662: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10166 = 595.0 / 10368.0 * t525 * t10164;
    let t10171 = t3358 * t219;
    let t10178 = t1257 * t1257;
    let t10179 = 1.0 / t10178;
    let t10180 = t73 * t10179;
    let t10204 = t1219 * t3357;
    let t10232 = t3387 * t1270;
    let t10236 = t3202 * t3205;
    let t10281 = 4.0 * t7651;
    let t10282 = 12.0 * t7653;
    let t10283 = 48.0 * t7660;
    let t10284 = 80.0 * t7662;
    (t10166, t10171, t10178, t10179, t10180, t10204, t10232, t10236, t10281, t10282, t10283, t10284)
}
