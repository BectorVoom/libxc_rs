//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1077/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1077<F: Float>(t27051: F, t539: F, t1323: F, t7918: F, t1385: F, t7936: F, t3887: F, t1375: F, t1386: F, t16030: F, t2092: F, t24071: F, t26217: F, t26335: F, t26340: F, t26345: F, t26352: F, t26357: F, t27009: F, t3882: F, t568: F, t7925: F) -> (F, F, F, F) {
    let t27052 = t539 * t27051;
    let t27059 = t1323 * t7918;
    let t27061 = t7936 * t1385;
    let t27062 = t3887 * t27061;
    let t27065 = F::new(0.3289868133696452873e-1) * t26217 - t24071 + F::new(2.0) * t3882 * t7925 - t27009 * t1386 - t16030 * t2092 + t27052 * t568 + F::new(0.9869604401089358619e-1) * t26335 + F::new(0.3289868133696452873e-1) * t26340 + F::new(0.82246703342411321825e-2) * t26345 - F::new(0.16449340668482264365e-1) * t26352 + F::new(0.3289868133696452873e-1) * t26357 + t27059 * t568 + F::new(2.0) * t1375 * t27062;
    (t27052, t27059, t27062, t27065)
}
