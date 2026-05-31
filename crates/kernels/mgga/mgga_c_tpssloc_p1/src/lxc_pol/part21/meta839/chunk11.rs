//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3012/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3012<F: Float>(t1057: F, t61729: F, t3199: F, t61734: F, t1061: F, t11037: F, t11051: F, t11065: F, t14574: F, t14581: F, t14590: F, t14591: F, t14608: F, t14618: F, t14623: F, t14627: F, t18131: F, t18138: F, t3040: F, t3186: F, t3202: F, t43553: F, t43554: F, t4677: F, t47857: F, t5928: F, t5932: F, t5933: F, t5936: F) -> F {
    let t62994 = t61729 * t1057;
    let t63004 = t61734 * t3199;
    let t63022 = -F::cast_from(36.0_f64) * t3040 * t43553 * t43554 * t5928 - F::cast_from(12.0_f64) * t11065 * t14590 * t5932 - F::cast_from(6.0_f64) * t11065 * t14590 * t5936 + F::cast_from(8.0_f64) * t18138 * t3186 * t4677 + F::cast_from(2.0_f64) * t1061 * t62994 - F::cast_from(4.0_f64) * t11037 * t18131 + F::cast_from(2.0_f64) * t11051 * t5933 - F::cast_from(4.0_f64) * t14574 * t14608 + F::cast_from(8.0_f64) * t14581 * t14618 - F::cast_from(12.0_f64) * t14591 * t47857 - F::cast_from(2.0_f64) * t14608 * t14623 - F::cast_from(2.0_f64) * t14608 * t14627 - t3202 * t63004;
    t63022
}
