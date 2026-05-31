//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2201/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2201<F: Float>(t90701: F, t1985: F, t22662: F, t26193: F, t81284: F, t26203: F, t6883: F, t6897: F, t7700: F, t80645: F, t12030: F, t1375: F, t1385: F, t16022: F, t16436: F, t16474: F, t16475: F, t1843: F, t2015: F, t2016: F, t26224: F, t26348: F, t26371: F, t26471: F, t26477: F, t3758: F, t3887: F, t3912: F, t55069: F, t55134: F, t6958: F, t6993: F, t7750: F, t81282: F, t81319: F, t90687: F, t90690: F, t90696: F) -> F {
    let t90702 = F::cast_from(0.82246703342411321824e-2_f64) * t90701;
    let t90704 = t1985 * t26193 * t22662;
    let t90706 = F::cast_from(0.3289868133696452873e-1_f64) * t81284;
    let t90707 = t6883 * t26203;
    let t90708 = F::cast_from(0.38381794893125283518e-1_f64) * t90707;
    let t90723 = t6897 * t80645 * t7700;
    let t90724 = F::cast_from(0.82246703342411321824e-2_f64) * t90723;
    let t90725 = F::cast_from(2.0_f64) * t1375 * t3887 * t2015 * t16436 - t90687 - F::cast_from(0.82246703342411321825e-2_f64) * t90690 + F::cast_from(4.0_f64) * t1375 * t3887 * t26471 * t1385 + t81282 + F::cast_from(24.0_f64) * t26224 * t90696 * t16474 + t90702 - F::cast_from(0.82246703342411321825e-2_f64) * t90704 + t90706 + t90708 - t26477 * t3912 - t81319 * t1843 + F::cast_from(4.0_f64) * t3758 * t26348 - t55069 * t2016 - t55134 * t2016 - F::cast_from(2.0_f64) * t16022 * t6993 - t12030 * t7750 + F::cast_from(4.0_f64) * t3758 * t26371 - F::cast_from(6.0_f64) * t6958 * t16475 + t90724;
    t90725
}
