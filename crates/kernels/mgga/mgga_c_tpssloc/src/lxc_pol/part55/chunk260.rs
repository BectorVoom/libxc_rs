//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 260/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk260<F: Float>(t1111: F, t241: F, t457: F, t1090: F, t136: F, t1092: F, t1103: F, t1105: F, t1108: F, t422: F, t1099: F, t1086: F, t432: F, t427: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1112 = 0.82156666666666666667e-1 * t1111;
    let t1113 = t241 * t457;
    let t1114 = t1113 * t1090;
    let t1115 = t136 * t1114;
    let t1117 = 0.1898925e1 * t1103 - t1105 + 0.29896666666666666667e0 * t1092 + 0.3071625e0 * t1108 - t1112 + 0.82156666666666666667e-1 * t1115;
    let t1118 = 1.0 / t422;
    let t1119 = t1117 * t1118;
    let t1121 = 1.0 * t1099 * t1119;
    let t1122 = 0.17123333333333333333e-1 * t1086;
    let t1124 = -t1122 + 0.17123333333333333333e-1 * t1092;
    let t1127 = t432 * t432;
    let t1128 = 1.0 / t1127;
    let t1129 = t427 * t1128;
    let t1131 = 0.516475e0 * t1086;
    let t1134 = 0.104195e0 * t1111;
    let t1136 = 0.3529725e1 * t1103 - t1131 + 0.516475e0 * t1092 + 0.6311625e0 * t1108 - t1134 + 0.104195e0 * t1115;
    (t1112, t1113, t1114, t1115, t1117, t1118, t1119, t1121, t1122, t1124, t1127, t1128, t1129, t1131, t1134, t1136)
}
