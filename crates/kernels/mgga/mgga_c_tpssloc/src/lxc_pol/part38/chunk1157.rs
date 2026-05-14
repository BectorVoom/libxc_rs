//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1157/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1157<F: Float>(t16451: F, t16485: F, t3734: F, t571: F, t1390: F, t5356: F, t12127: F, t12133: F, t12141: F, t12466: F, t1297: F, t1307: F, t15983: F, t15985: F, t15987: F, t15988: F, t16018: F, t16165: F, t16166: F, t16167: F, t16168: F, t16171: F, t16172: F, t1799: F, t193: F, t3918: F, t533: F, t9853: F, t9859: F) -> (F,) {
    let t16486 = t16451 + t16485;
    let t16490 = t3734 * t571;
    let t16497 = t5356 * t1390;
    let t16501 = t1390 * t16486 * t193 * t533 + 3.0 * t12466 * t1799 * t3918 + 3.0 * t1297 * t16018 * t193 + 6.0 * t1307 * t16497 * t3918 + 6.0 * t16490 * t1799 * t193 + t12127 + t12133 - t12141 + t15983 + t15985 - t15987 + t15988 + t16165 - t16166 + t16167 + t16168 - t16171 - t16172 + t9853 + t9859;
    (t16501,)
}
