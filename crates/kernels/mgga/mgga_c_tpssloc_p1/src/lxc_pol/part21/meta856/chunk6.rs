//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3102/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3102<F: Float>(t43959: F, t6024: F, t1128: F, t18668: F, t3263: F, t5983: F, t3266: F, t1129: F, t1137: F, t1138: F, t11410: F, t15118: F, t15141: F, t1683: F, t18840: F, t18894: F, t3327: F, t3352: F, t3360: F, t44211: F, t4797: F, t4820: F, t51594: F, t6037: F, t6053: F, t63763: F, t63765: F, t63767: F, t63769: F, t63771: F, t63829: F, t64100: F, t64103: F, t64132: F, t64148: F, t64165: F, t64181: F, t64197: F, t64212: F, t64229: F, t64245: F) -> (F, F, F) {
    let t64253 = F::cast_from(0.16081979498692535067e2_f64) * t43959 * t6024;
    let t64254 = t18668 * t1128;
    let t64257 = t5983 * t3263;
    let t64259 = F::cast_from(2.0_f64) * t64257 * t3266;
    let t64260 = -t63763 - t63765 + t63767 - t63769 - t63771 + t63829 - t64100 + F::cast_from(1.0_f64) * t18840 * t3352 + F::cast_from(0.32163958997385070134e2_f64) * t64103 * t3360 + F::cast_from(2.0_f64) * t51594 * t1683 + F::cast_from(4.0_f64) * t15141 * t4820 + F::cast_from(2.0_f64) * t4797 * t15118 - F::cast_from(2.0_f64) * t44211 * t6037 + F::cast_from(1.0_f64) * t11410 * t6053 + F::cast_from(2.0_f64) * t3327 * t18894 + F::cast_from(1.0_f64) * t1129 * (t64132 + t64148 + t64165 + t64181 + t64197 + t64212 + t64229 + t64245) * t1137 - t64253 + F::cast_from(2.0_f64) * t64254 * t1138 + t64259;
    (t64253, t64259, t64260)
}
