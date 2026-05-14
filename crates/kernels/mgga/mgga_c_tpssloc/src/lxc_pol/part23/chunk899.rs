//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 899/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk899<F: Float>(t15909: F, t12044: F, t12046: F, t12048: F, t12053: F, t12055: F, t12057: F, t12059: F, t12087: F, t20372: F, t20398: F, t9780: F, t9789: F, t19682: F, t15972: F, t12094: F, t12103: F, t12105: F, t12109: F, t12114: F, t12116: F, t9793: F, t9797: F, t9820: F, t9824: F) -> (F, F, F, F, F) {
    let t20520 = 0.32530743900905219526e-1 * t15909;
    let t20521 = -t20372 + t9780 + t20398 + t20520 - t12044 - t12046 - t12048 + t12053 - t12055 - t12057 - t12059 - t9789 + t12087;
    let t20523 = 0.17544670867903938621e1 * t19682;
    let t20524 = 3.0 * t15972;
    let t20525 = -t12094 + t9793 + t9797 - t9820 - t9824 - t20523 + t20524 + t12103 - t12105 - t12109 - t12114 + t12116;
    (t20520, t20521, t20523, t20524, t20525)
}
