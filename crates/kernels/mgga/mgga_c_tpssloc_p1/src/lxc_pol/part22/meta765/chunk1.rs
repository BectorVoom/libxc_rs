//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2585/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2585<F: Float>(t19047: F, t4997: F, t19040: F, t5005: F, t71095: F, t71097: F, t71106: F, t71109: F, t71112: F, t71114: F, t71118: F, t71217: F, t71221: F, t71225: F, t71227: F, t71230: F, t71233: F, t71236: F, t71238: F, t71241: F, t71245: F, t71247: F, t71249: F, t71251: F) -> (F, F, F) {
    let t72181 = t19047 * t4997;
    let t72183 = t5005 * t19040;
    let t72195 = t71095 - t71097 + t71106 - t71109 - t71112 + t71114 + t71118 - t71217 + t71221 - t71225 + t71227 + t71230 - t71233 - t71236 + t71238 - t71241 + t71245 - t71247 - t71249 - t71251;
    (t72181, t72183, t72195)
}
