//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1976/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1976<F: Float>(t87165: F, t87177: F, t26653: F, t814: F, t87520: F, t1509: F, t7084: F, t87522: F, t13171: F, t1510: F, t24256: F, t2617: F, t26598: F, t26662: F, t4166: F, t4291: F, t7101: F, t812: F, t81615: F, t81617: F, t829: F, t84945: F, t87171: F, t87174: F, t87517: F, t87527: F) -> (F, F) {
    let t92530 = F::cast_from(0.3289868133696452873e-1_f64) * t87165;
    let t92543 = F::cast_from(0.16449340668482264365e-1_f64) * t87177;
    let t92546 = t814 * t26653;
    let t92551 = F::cast_from(0.3289868133696452873e-1_f64) * t87520;
    let t92552 = t7084 * t1509;
    let t92556 = F::cast_from(0.15352717957250113407e0_f64) * t87522;
    let t92558 = t92530 + F::cast_from(0.3289868133696452873e-1_f64) * t81615 - t812 * t84945 * t1510 - F::cast_from(0.38381794893125283518e-1_f64) * t81617 + F::cast_from(2.0_f64) * t4166 * t24256 - F::cast_from(2.0_f64) * t2617 * t26662 - t812 * t7101 * t13171 + F::cast_from(0.6579736267392905746e-1_f64) * t87171 - F::cast_from(0.3289868133696452873e-1_f64) * t87174 + t92543 - F::cast_from(2.0_f64) * t2617 * t26598 - F::cast_from(2.0_f64) * t812 * t92546 * t829 + F::cast_from(0.3289868133696452873e-1_f64) * t87517 - t92551 - F::cast_from(2.0_f64) * t4291 * t92552 * t829 + t92556 - F::cast_from(0.3289868133696452873e-1_f64) * t87527;
    (t92552, t92558)
}
