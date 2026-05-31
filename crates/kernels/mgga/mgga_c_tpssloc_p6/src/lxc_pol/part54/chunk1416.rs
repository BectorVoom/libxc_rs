//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1416/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1416<F: Float>(t22704: F, t33249: F, t81326: F, t22633: F, t31550: F, t90566: F, t1985: F, t27114: F, t6889: F, t6906: F, t113963: F, t115508: F, t120270: F, t120274: F, t120277: F, t16022: F, t16439: F, t26224: F, t26347: F, t26989: F, t31653: F, t31655: F, t5354: F, t8637: F, t97626: F) -> F {
    let t122178 = t22704 * t81326 * t33249;
    let t122187 = t22633 * t90566 * t31550;
    let t122192 = t1985 * t6889 * t6906 * t27114;
    let t122196 = -F::cast_from(0.82246703342411321825e-2_f64) * t122178 - t16022 * t8637 + t120270 - F::cast_from(6.0_f64) * t26224 * t26989 * t26347 - t113963 + t120274 - t16439 * t8637 - t31653 * t5354 - t120277 + F::cast_from(0.16449340668482264365e-1_f64) * t122187 - F::cast_from(0.38381794893125283518e-1_f64) * t115508 - F::cast_from(0.82246703342411321825e-2_f64) * t122192 - F::cast_from(6.0_f64) * t97626 * t31655;
    t122196
}
