//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 844/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk844<F: Float>(t6168: F, t68: F, t484: F, t3560: F, t5392: F, t974: F, t1196: F, t5398: F, t3555: F, t1653: F, t1735: F, t3578: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6169 = t6168 * t68;
    let t6170 = t6169 * t484;
    let t6177 = t3560 * t5392;
    let t6178 = t974 * t6177;
    let t6183 = t1196 * t5398;
    let t6184 = t974 * t6183;
    let t6187 = t3555 * t5392;
    let t6188 = t974 * t6187;
    let t6191 = t1735 * t1653;
    let t6192 = t3578 * t6191;
    (t6169, t6170, t6177, t6178, t6183, t6184, t6187, t6188, t6191, t6192)
}
