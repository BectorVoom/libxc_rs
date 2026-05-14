//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1022/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1022<F: Float>(t11144: F, t285: F, t3907: F, t8833: F, t912: F, t2593: F, t3882: F, t905: F, t1448: F, t8749: F, t2595: F, t8752: F, t10982: F, t10989: F, t11049: F, t10992: F, t10994: F, t11041: F, t11044: F, t11047: F, t11051: F, t8647: F, t8796: F, t8797: F) -> (F, F, F, F, F, F) {
    let t11146 = 0.621814e-1 * t11144 * t285;
    let t11147 = t3907 * t8833;
    let t11149 = 0.17315859105681463759e2 * t912 * t11147;
    let t11152 = t2593 * t3882;
    let t11153 = t11152 * t905;
    let t11155 = 0.23392894490538584828e1 * t912 * t11153;
    let t11156 = t8749 * t1448;
    let t11157 = t8752 * t2595;
    let t11158 = t11156 * t11157;
    let t11160 = 0.10254018858216406658e4 * t912 * t11158;
    let t11169 = 0.20128333333333333334e0 * t10982;
    let t11172 = 0.11038e0 * t10989;
    let t11179 = 0.22076e0 * t11049;
    let t11181 = t11172 - 0.82785e-1 * t10992 - 0.91983333333333333334e-1 * t10994 - 0.11038e0 * t8647 - t8796 - t8797 + 0.258925e1 * t11041 - 0.49671e0 * t11044 + 0.16557e0 * t11047 - t11179 + 0.36793333333333333334e-1 * t11051;
    (t11146, t11149, t11155, t11160, t11169, t11181)
}
