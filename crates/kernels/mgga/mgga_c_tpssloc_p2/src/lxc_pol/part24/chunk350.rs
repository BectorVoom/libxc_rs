//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 350/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk350<F: Float>(t1190: F, t491: F, t1169: F, t221: F, t456: F, t1089: F, t1176: F, t607: F, t974: F, t225: F, t68: F, t484: F) -> (F, F, F, F, F, F, F, F) {
    let t1191 = t1190 * t491;
    let t1193 = t221 * t1169;
    let t1195 = t456 * t1193 / F::new(288.0);
    let t1196 = t1176 * t1089;
    let t1197 = t1196 * t607;
    let t1198 = t974 * t1197;
    let t1201 = t1190 * t225;
    let t1202 = t1201 * t68;
    let t1203 = t1202 * t484;
    (t1191, t1195, t1196, t1197, t1198, t1201, t1202, t1203)
}
