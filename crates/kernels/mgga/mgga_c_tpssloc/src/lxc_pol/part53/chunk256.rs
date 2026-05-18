//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 256/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk256<F: Float>(t1111: F, t241: F, t457: F, t1090: F, t136: F, t1092: F, t1103: F, t1105: F, t1108: F, t422: F, t1099: F, t1086: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1112 = F::new(0.82156666666666666667e-1) * t1111;
    let t1113 = t241 * t457;
    let t1114 = t1113 * t1090;
    let t1115 = t136 * t1114;
    let t1117 = F::new(0.1898925e1) * t1103 - t1105 + F::new(0.29896666666666666667e0) * t1092 + F::new(0.3071625e0) * t1108 - t1112 + F::new(0.82156666666666666667e-1) * t1115;
    let t1118 = F::new(1.0) / t422;
    let t1119 = t1117 * t1118;
    let t1121 = F::new(1.0) * t1099 * t1119;
    let t1122 = F::new(0.17123333333333333333e-1) * t1086;
    (t1112, t1113, t1114, t1115, t1117, t1118, t1119, t1121, t1122)
}
