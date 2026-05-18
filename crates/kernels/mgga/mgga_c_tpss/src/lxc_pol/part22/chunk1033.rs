//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1033/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1033<F: Float>(t11156: F, t11157: F, t912: F, t10982: F, t10989: F, t11049: F, t10992: F, t10994: F, t11041: F, t11044: F, t11047: F, t11051: F, t8647: F, t8796: F, t8797: F) -> (F, F, F) {
    let t11158 = t11156 * t11157;
    let t11160 = F::new(0.10254018858216406658e4) * t912 * t11158;
    let t11169 = F::new(0.20128333333333333334e0) * t10982;
    let t11172 = F::new(0.11038e0) * t10989;
    let t11179 = F::new(0.22076e0) * t11049;
    let t11181 = t11172 - F::new(0.82785e-1) * t10992 - F::new(0.91983333333333333334e-1) * t10994 - F::new(0.11038e0) * t8647 - t8796 - t8797 + F::new(0.258925e1) * t11041 - F::new(0.49671e0) * t11044 + F::new(0.16557e0) * t11047 - t11179 + F::new(0.36793333333333333334e-1) * t11051;
    (t11160, t11169, t11181)
}
