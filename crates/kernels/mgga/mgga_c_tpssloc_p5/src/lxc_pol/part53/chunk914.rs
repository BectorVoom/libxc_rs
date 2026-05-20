//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 914/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk914<F: Float>(t1378: F, t33843: F, t1375: F, t1843: F, t2092: F, t27009: F, t27068: F, t32120: F, t32173: F, t32183: F, t33308: F, t33311: F, t5215: F, t5321: F, t7194: F, t7937: F, t8794: F, t8801: F) -> (F, F) {
    let t33844 = t1378 * t33843;
    let t33852 = -t32120 * t1843 - F::new(2.0) * t27009 * t2092 - t5215 * t8801 - t5321 * t8801 - F::cast_from(0.6579736267392905746e-1_f64) * t33308 - F::cast_from(0.3289868133696452873e-1_f64) * t33311 - F::new(2.0) * t7194 * t7937 + t32173 - t32183 - t1375 * t33844 + F::new(2.0) * t5215 * t8794 + F::new(2.0) * t5321 * t8794 - F::new(2.0) * t27068 * t2092;
    (t33844, t33852)
}
