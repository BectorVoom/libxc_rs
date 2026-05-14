//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1273/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1273<F: Float>(t3087: F, t6504: F, t12429: F, t19075: F, t2713: F, t6005: F, t12441: F, t6007: F, t12475: F, t19084: F, t12367: F, t6013: F, t3025: F, t6496: F, t12317: F, t12384: F, t6002: F) -> (F, F, F, F, F, F, F, F, F) {
    let t68408 = t6504 * t3087;
    let t68413 = t2713 * t19075 * t12429;
    let t68417 = t2713 * t6005 * t12429;
    let t68423 = t6007 * t12441 / 1152.0;
    let t68438 = t19084 * t12475 / 1728.0;
    let t68464 = t6013 * t12367;
    let t68466 = t6496 * t3025;
    let t68469 = t19084 * t12317 / 1728.0;
    let t68472 = t6002 * t12384;
    (t68408, t68413, t68417, t68423, t68438, t68464, t68466, t68469, t68472)
}
