//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1082/1190 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1082<F: Float>(t3378: F, t4882: F, t1164: F, t3411: F, t4879: F, t11433: F, t3396: F, t4874: F, t11424: F, t4745: F, t11185: F, t4786: F, t1117: F, t4782: F, t3264: F, t1671: F, t3307: F) -> (F, F, F, F, F, F, F, F) {
    let t15036 = t4882 * t3378;
    let t15038 = 0.35089341735807877242e1 * t1164 * t15036;
    let t15040 = 0.11696447245269292414e1 * t3411 * t4879;
    let t15041 = t4882 * t11433;
    let t15043 = 0.17315859105681463759e2 * t1164 * t15041;
    let t15044 = t4874 * t3396;
    let t15046 = 0.11696447245269292414e1 * t1164 * t15044;
    let t15048 = 4.0 * t11424 * t4745;
    let t15050 = 0.32163958997385070134e2 * t11185 * t4786;
    let t15051 = t4782 * t1117;
    let t15053 = 4.0 * t3264 * t15051;
    let t15054 = t1671 * t3307;
    (t15038, t15040, t15043, t15046, t15048, t15050, t15053, t15054)
}
