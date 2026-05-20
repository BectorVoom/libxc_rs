//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3089/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3089<F: Float>(t1107: F, t63996: F, t1102: F, t4756: F, t14804: F, t14801: F, t3270: F, t64008: F, t1113: F, t136: F, t63353: F, t43780: F, t43782: F, t43816: F, t44053: F, t50968: F, t50970: F, t50972: F, t50978: F, t51039: F, t51041: F) -> (F, F, F, F, F, F) {
    let t64028 = t1107 * t63996;
    let t64030 = t1102 * t4756;
    let t64031 = t14804 * t64030;
    let t64033 = t14801 * t64030;
    let t64042 = t3270 * t64008;
    let t64045 = t136 * t1113 * t63353;
    let t64049 = F::new(0.3071625e0) * t64028 - F::new(0.3071625e0) * t64031 + F::new(0.5696775e1) * t64033 + F::cast_from(0.73028148148148148146e-1_f64) * t50968 + F::cast_from(0.36514074074074074073e-1_f64) * t50970 + F::cast_from(0.21908444444444444444e0_f64) * t50972 + t44053 - F::cast_from(0.48685432098765432097e-1_f64) * t50978 + F::cast_from(0.13287407407407407408e0_f64) * t43780 + F::cast_from(0.26574814814814814816e0_f64) * t43782 - F::cast_from(0.62007901234567901237e0_f64) * t43816 - F::new(0.1898925e1) * t64042 + F::cast_from(0.82156666666666666667e-1_f64) * t64045 + F::cast_from(0.73028148148148148147e0_f64) * t51039 - F::cast_from(0.21908444444444444444e0_f64) * t51041;
    (t64028, t64031, t64033, t64042, t64045, t64049)
}
