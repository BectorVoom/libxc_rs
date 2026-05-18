//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1033/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1033<F: Float>(t117173: F, t122168: F, t122178: F, t1323: F, t1375: F, t16439: F, t1842: F, t1843: F, t2092: F, t27115: F, t32120: F, t32150: F, t32151: F, t32156: F, t33804: F, t33822: F, t3882: F, t3887: F, t5215: F, t5321: F, t5326: F, t5354: F, t568: F, t7194: F, t8794: F, t93313: F, t93338: F) -> F {
    let t124069 = -F::new(0.3289868133696452873e-1) * t122168 - F::new(2.0) * t7194 * t27115 - t117173 * t1843 - t5321 * t32151 + F::new(2.0) * t16439 * t8794 - t32120 * t5354 + F::new(2.0) * t1375 * t3887 * t32150 * t1842 + F::new(4.0) * t3882 * t33804 - F::new(0.3289868133696452873e-1) * t122178 - F::new(2.0) * t93338 * t2092 + t1323 * t33822 * t568 + F::new(4.0) * t5321 * t32156 - F::new(2.0) * t93313 * t2092 + F::new(4.0) * t5215 * t32156 + F::new(2.0) * t32120 * t5326;
    t124069
}
