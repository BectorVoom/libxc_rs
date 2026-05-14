//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1066/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1066<F: Float>(t15615: F, t17686: F, t4582: F, t1155: F, t6069: F, t1695: F, t4857: F, t6088: F, t6085: F, t3403: F, t6084: F, t4861: F, t11285: F, t6068: F, t11310: F, t11365: F, t15126: F, t15136: F, t15146: F, t15207: F, t18247: F, t3376: F, t3401: F, t4802: F, t4824: F, t4840: F, t4862: F) -> (F, F) {
    let t18593 = t15615 * t17686;
    let t18594 = t4582 * t18593;
    let t18603 = t6069 * t1155;
    let t18606 = t1695 * t4857;
    let t18609 = t6088 * t1155;
    let t18612 = t6085 * t1155;
    let t18615 = t6084 * t3403;
    let t18616 = t18615 * t1155;
    let t18619 = t4861 * t4857;
    let t18622 = t6068 * t11285;
    let t18623 = t18622 * t1155;
    let t18630 = -0.23392894490538584828e1 * t15136 * t4840 + 0.34631718211362927517e2 * t15126 * t4862 + 0.35089341735807877242e1 * t3401 * t18603 - 0.23392894490538584828e1 * t3376 * t18606 - 0.10389515463408878255e3 * t11365 * t18609 - 0.11696447245269292414e1 * t3376 * t18612 + 0.17315859105681463759e2 * t3401 * t18616 + 0.34631718211362927518e2 * t3401 * t18619 + 0.10254018858216406658e4 * t11310 * t18623 + t18247 - 4.0 * t15207 * t4802 + 0.64327917994770140268e2 * t15146 * t4824;
    (t18594, t18630)
}
