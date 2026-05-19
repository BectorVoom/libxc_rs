//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1253/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1253<F: Float>(t6897: F, t8458: F, t90544: F, t114154: F, t114172: F, t22892: F, t7691: F, t114160: F, t1985: F, t7700: F, t114174: F, t22666: F, t32697: F) -> (F, F, F, F, F, F) {
    let t120296 = t6897 * t90544 * t8458;
    let t120297 = F::cast_from(0.82246703342411321825e-2_f64) * t120296;
    let t120304 = F::cast_from(0.82246703342411321825e-2_f64) * t114154;
    let t120308 = t22892 * t114172 * t7691;
    let t120309 = F::cast_from(0.16449340668482264365e-1_f64) * t120308;
    let t120312 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t114160 * t7700;
    let t120313 = F::cast_from(0.82246703342411321825e-2_f64) * t114174;
    let t120316 = F::cast_from(0.16449340668482264365e-1_f64) * t1985 * t22666 * t32697;
    (t120297, t120304, t120309, t120312, t120313, t120316)
}
