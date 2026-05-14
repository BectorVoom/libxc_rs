//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 395/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk395<F: Float>(t1735: F, t248: F, t3570: F, t1213: F, t1009: F, t1720: F, t1011: F, t1212: F, t1226: F, t1730: F, t1017: F, t1742: F, t1210: F, t1207: F, t372: F, t479: F) -> (F, F, F, F, F, F) {
    let t4997 = t248 * t3570 * t1735;
    let t4998 = t1213 * t4997;
    let t5000 = t1720 * t1009;
    let t5001 = t5000 * t1011;
    let t5002 = t5001 * t1212;
    let t5005 = t1730 * t1226;
    let t5017 = t1742 * t1017;
    let t5018 = t1210 * t5017;
    let t5019 = t1207 * t5018;
    let t5022 = t1742 * t372;
    let t5023 = t479 * t5022;
    (t4998, t5000, t5002, t5005, t5019, t5023)
}
