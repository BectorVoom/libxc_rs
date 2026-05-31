//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2318/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2318<F: Float>(t28719: F, t3216: F, t100489: F, t1068: F, t1070: F, t1637: F, t18169: F, t193: F, t23738: F, t23742: F, t25840: F, t25845: F, t336: F, t4696: F, t4700: F, t5946: F, t5950: F, t6822: F, t83472: F, t83479: F, t89698: F, t89702: F, t99104: F, t99143: F, t99172: F, t99202: F, t99238: F, t99271: F, t99313: F, t99353: F, t99390: F, t99422: F, t99450: F, t99866: F, t99894: F, t99930: F, t99959: F) -> F {
    let t100497 = t28719 * t3216;
    let t100528 = t193 * t336 * (t99104 + t99143 + t99172 + t99202 + t99238 + t99271 + t99313 + t99353 + t99390 + t99422 + t99450 + t99866 + t99894 + t99930 + t99959 + t100489) * t1070 - t4700 * t100497 * t1068 - F::cast_from(2.0_f64) * t4700 * t89698 * t1637 + F::cast_from(4.0_f64) * t4700 * t89702 * t25845 - F::cast_from(2.0_f64) * t4700 * t25840 * t4696 + F::cast_from(2.0_f64) * t4700 * t83472 * t5950 - F::cast_from(6.0_f64) * t4700 * t83479 * t5950 * t1068 + F::cast_from(4.0_f64) * t4700 * t23742 * t1637 * t4696 - t4700 * t23738 * t5946 + F::cast_from(2.0_f64) * t4700 * t23742 * t5946 * t1068 - t4700 * t6822 * t18169;
    t100528
}
