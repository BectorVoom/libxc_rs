//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1113/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1113<F: Float>(t12652: F, t4972: F, t4582: F, t11153: F, t3584: F, t14165: F, t1734: F, t3508: F, t1089: F, t1215: F, t607: F, t3578: F, t1196: F, t12606: F, t974: F, t3548: F, t4889: F) -> (F, F, F, F, F) {
    let t15649 = t4972 * t12652;
    let t15650 = t4582 * t15649;
    let t15654 = t3584 * t11153;
    let t15655 = t15654 * t14165;
    let t15656 = t4582 * t15655;
    let t15659 = t1734 * t3508;
    let t15660 = t1215 * t1089;
    let t15661 = t15660 * t607;
    let t15662 = t15659 * t15661;
    let t15663 = t3578 * t15662;
    let t15666 = t1196 * t12606;
    let t15667 = t974 * t15666;
    let t15671 = t4889 * t3548 / 162.0;
    (t15650, t15656, t15663, t15667, t15671)
}
