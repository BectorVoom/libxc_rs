//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 890/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk890<F: Float>(t1378: F, t32150: F, t31595: F, t2091: F, t3887: F, t7213: F, t1375: F, t1386: F, t2092: F, t24082: F, t24095: F, t31552: F, t31561: F, t31591: F, t32120: F, t32127: F, t3758: F, t3882: F, t7194: F, t7214: F, t8801: F) -> (F, F, F, F) {
    let t32151 = t1378 * t32150;
    let t32154 = F::cast_from(0.16449340668482264365e-1_f64) * t31595;
    let t32156 = t3887 * t2091 * t7213;
    let t32159 = F::cast_from(0.6579736267392905746e-1_f64) * t31552 - F::cast_from(2.0_f64) * t7194 * t7214 - t32120 * t1386 - F::cast_from(2.0_f64) * t24095 * t2092 - F::cast_from(2.0_f64) * t24082 * t2092 + F::cast_from(0.6579736267392905746e-1_f64) * t31561 - t32127 - t3758 * t8801 - t3882 * t8801 - t1375 * t32151 + F::cast_from(0.3289868133696452873e-1_f64) * t31591 + t32154 + F::cast_from(4.0_f64) * t1375 * t32156;
    (t32151, t32154, t32156, t32159)
}
