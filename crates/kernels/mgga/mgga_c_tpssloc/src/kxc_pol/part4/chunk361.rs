//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 361/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk361<F: Float>(t1190: F, t491: F, t1169: F, t221: F, t456: F, t1089: F, t1176: F) -> (F, F, F, F) {
    let t1191 = t1190 * t491;
    let t1193 = t221 * t1169;
    let t1195 = t456 * t1193 / 288.0;
    let t1196 = t1176 * t1089;
    (t1191, t1193, t1195, t1196)
}
