//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1050/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1050<F: Float>(t1117: F, t18265: F, t11275: F, t3411: F, t6106: F, t1157: F, t6105: F, t1164: F, t11282: F, t6068: F, t11285: F, t1155: F, t11292: F, t4883: F, t15218: F, t4882: F) -> (F, F, F, F, F, F) {
    let t18266 = t18265 * t1117;
    let t18268 = 0.51726012919273400301e3 * t11275 * t18266;
    let t18270 = 0.17315859105681463759e2 * t3411 * t6106;
    let t18271 = t6105 * t1157;
    let t18273 = 0.35089341735807877242e1 * t1164 * t18271;
    let t18274 = t11282 * t6068;
    let t18275 = t11285 * t1155;
    let t18276 = t18274 * t18275;
    let t18278 = 0.10254018858216406658e4 * t1164 * t18276;
    let t18279 = t11292 * t6068;
    let t18280 = t18279 * t4883;
    let t18282 = 0.10389515463408878255e3 * t1164 * t18280;
    let t18283 = t4882 * t15218;
    (t18268, t18270, t18273, t18278, t18282, t18283)
}
