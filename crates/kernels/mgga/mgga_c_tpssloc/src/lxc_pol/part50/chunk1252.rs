//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1252/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1252<F: Float>(t1992: F, t31091: F, t90566: F, t32698: F, t6883: F, t113946: F, t1842: F, t22635: F, t32705: F, t81159: F, t113963: F, t12021: F, t1375: F, t1385: F, t16030: F, t16460: F, t31096: F, t31131: F, t32686: F, t32690: F, t32757: F, t3758: F, t3882: F, t3887: F, t5215: F, t5321: F, t5353: F, t8475: F, t8476: F, t8485: F, t8486: F) -> F {
    let t120258 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t90566 * t31091;
    let t120269 = t6883 * t32698;
    let t120270 = F::cast_from(0.38381794893125283518e-1_f64) * t120269;
    let t120274 = F::cast_from(0.3289868133696452873e-1_f64) * t1992 * t22635 * t113946 * t1842;
    let t120276 = t81159 * t32705;
    let t120277 = F::cast_from(0.76763589786250567037e-1_f64) * t120276;
    let t120292 = -F::new(6.0) * t12021 * t1375 * t5353 * t8475 + F::new(2.0) * t1375 * t1385 * t32757 * t3887 + F::new(2.0) * t1375 * t3887 * t5353 * t8485 + F::new(2.0) * t16030 * t8476 - t16460 * t8486 + F::new(4.0) * t31096 * t5215 + F::new(4.0) * t31096 * t5321 + F::new(2.0) * t31131 * t5321 + F::new(2.0) * t32686 * t3758 - F::new(6.0) * t32690 * t3882 - t113963 + t120258 + t120270 + t120274 - t120277;
    t120292
}
