//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 2006/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk2006<F: Float>(t27143: F, t532: F, t90459: F, t90468: F, t90470: F, t90472: F, t225: F, t27137: F, t27059: F, t2091: F, t40590: F, t1386: F, t16474: F, t24082: F, t26224: F, t5354: F, t80647: F, t80659: F, t80663: F, t80665: F, t80667: F, t80671: F, t90462: F, t90466: F, t90477: F, t90485: F, t90491: F, t90498: F) -> (F, F) {
    let t93286 = t532 * t27143;
    let t93306 = F::cast_from(0.76763589786250567036e-1_f64) * t90459;
    let t93309 = F::cast_from(0.15352717957250113407e0_f64) * t90468;
    let t93310 = F::cast_from(0.15352717957250113407e0_f64) * t90470;
    let t93311 = F::cast_from(0.15352717957250113407e0_f64) * t90472;
    let t93313 = t27137 * t225;
    let t93316 = t27059 * t225;
    let t93319 = t40590 * t2091;
    let t93332 = -F::cast_from(2.0_f64) * t24082 * t5354 + t93306 + F::cast_from(0.6579736267392905746e-1_f64) * t90462 + F::cast_from(0.3289868133696452873e-1_f64) * t90466 + t93309 + t93310 - t93311 + F::cast_from(0.6579736267392905746e-1_f64) * t90477 - F::cast_from(2.0_f64) * t93313 * t1386 - F::cast_from(2.0_f64) * t93316 * t1386 + F::cast_from(24.0_f64) * t26224 * t93319 * t16474 + F::cast_from(0.16449340668482264365e-1_f64) * t80647 - F::cast_from(0.9869604401089358619e-1_f64) * t90485 + F::cast_from(0.16449340668482264365e-1_f64) * t80659 - F::cast_from(0.6579736267392905746e-1_f64) * t90491 - F::cast_from(0.25587863262083522346e0_f64) * t80663 + F::cast_from(0.15352717957250113407e0_f64) * t80665 + F::cast_from(0.76763589786250567036e-1_f64) * t80667 - F::cast_from(0.20835831513410868196e0_f64) * t80671 - F::cast_from(0.46058153871750340222e0_f64) * t90498;
    (t93286, t93332)
}
