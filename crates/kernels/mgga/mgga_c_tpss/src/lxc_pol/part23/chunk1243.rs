//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1243/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1243<F: Float>(t1206: F, t4519: F, t1364: F, t2436: F, t10514: F, t3610: F, t821: F, t3724: F, t750: F, t2433: F, t14179: F, t782: F, t3664: F, t783: F, t1151: F, t4325: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44070 = t1206 * t4519;
    let t44169 = t2436 * t1364;
    let t44170 = t44169 * t10514;
    let t44329 = t3610 * t821;
    let t44350 = t3724 * t821;
    let t44470 = t750 * t3724;
    let t44474 = t1364 * t2433;
    let t44584 = t14179 * t782;
    let t44610 = t783 * t3664;
    let t50434 = t1151 * t4325;
    (t44070, t44170, t44329, t44350, t44470, t44474, t44584, t44610, t50434)
}
