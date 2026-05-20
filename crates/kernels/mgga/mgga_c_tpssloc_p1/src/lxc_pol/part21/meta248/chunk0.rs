//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1451/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1451<F: Float>(t479: F, t6163: F, t471: F, t225: F, t6150: F, t68: F, t484: F, t3560: F, t5392: F, t974: F, t1196: F, t5398: F) -> (F, F, F, F, F, F, F, F) {
    let t6164 = t479 * t6163;
    let t6165 = t471 * t6164;
    let t6168 = t6150 * t225;
    let t6169 = t6168 * t68;
    let t6170 = t6169 * t484;
    let t6177 = t3560 * t5392;
    let t6178 = t974 * t6177;
    let t6183 = t1196 * t5398;
    (t6164, t6165, t6168, t6169, t6170, t6177, t6178, t6183)
}
