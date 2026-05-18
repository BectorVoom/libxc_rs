//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 1020/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk1020<F: Float>(t225: F, t31573: F, t1985: F, t22666: F, t31607: F, t31590: F, t6883: F, t114145: F, t114150: F, t114155: F, t114159: F, t114163: F, t114168: F, t114171: F, t114175: F, t115469: F, t1386: F, t2016: F, t22630: F, t22905: F, t31642: F, t31653: F, t3882: F, t3889: F, t539: F, t568: F, t7194: F, t84655: F) -> F {
    let t115519 = t31573 * t225;
    let t115523 = t1985 * t22666 * t31607;
    let t115530 = t6883 * t31590;
    let t115532 = -F::new(6.0) * t7194 * t22630 - t7194 * t22905 + t539 * t115469 * t568 + t114145 - F::new(2.0) * t115519 * t1386 - t114150 - F::new(0.16449340668482264365e-1) * t115523 + t114155 + t114159 - t114163 - t84655 * t2016 - t114168 - t114171 - F::new(2.0) * t3882 * t31642 + t114175 + F::new(2.0) * t31653 * t3889 - F::new(0.38381794893125283518e-1) * t115530;
    t115532
}
