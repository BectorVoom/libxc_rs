//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 524/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk524<F: Float>(t116: F, t206: F, t212: F, t2586: F, t225: F, t799: F, t154: F, t2559: F, t222: F, t2563: F, t805: F, t68: F, t808: F) -> (F, F, F, F, F, F, F) {
    let t2587 = t206 * t116;
    let t2588 = t2587 * t212;
    let t2590 = 0.83333333333333333332e-3 * t2586 * t2588;
    let t2597 = t799 * t225;
    let t2600 = t2559 * t154;
    let t2602 = 35.0 / 432.0 * t2600 * t222;
    let t2603 = t2563 * t805;
    let t2617 = t808 * t68;
    (t2588, t2590, t2597, t2600, t2602, t2603, t2617)
}
