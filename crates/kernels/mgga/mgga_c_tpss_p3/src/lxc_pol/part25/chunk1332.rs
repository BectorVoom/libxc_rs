//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1332/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1332<F: Float>(t114: F, t71158: F, t117: F, t1279: F, t1281: F, t16052: F, t16067: F, t16076: F, t1668: F, t1670: F, t1851: F, t1853: F, t20660: F, t20678: F, t20679: F, t20685: F, t20691: F, t20694: F, t21958: F, t21975: F, t21981: F, t4549: F, t4556: F, t4637: F, t4674: F, t547: F, t5815: F, t6446: F, t6452: F, t6455: F) -> (F, F) {
    let t115 = F::cast_from(1.0_f64) < t114;
    let t71159 = piecewise3::<F>(t115, F::cast_from(0.0_f64), t71158);
    let t71181 = F::cast_from(3.0_f64) * t117 * t547 * t71159 + F::cast_from(6.0_f64) * t20678 * t4674 * t547 + F::cast_from(6.0_f64) * t4637 * t547 * t5815 + F::cast_from(12.0_f64) * t1279 * t21975 + F::cast_from(3.0_f64) * t1279 * t21981 + F::cast_from(3.0_f64) * t1281 * t21958 + F::cast_from(3.0_f64) * t16052 * t1853 + F::cast_from(12.0_f64) * t16067 * t1851 + F::cast_from(3.0_f64) * t16076 * t1851 + F::cast_from(12.0_f64) * t1668 * t20679 + F::cast_from(12.0_f64) * t1668 * t20685 + F::cast_from(12.0_f64) * t1668 * t20691 + F::cast_from(6.0_f64) * t1668 * t20694 + F::cast_from(6.0_f64) * t1670 * t20660 + F::cast_from(12.0_f64) * t4549 * t6452 + F::cast_from(6.0_f64) * t4549 * t6455 + F::cast_from(12.0_f64) * t4556 * t6446;
    (t71159, t71181)
}
