//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 885/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk885<F: Float>(t186: F, t660: F, t755: F, t730: F, t2376: F, t339: F, t795: F, t803: F, t207: F, t237: F, t235: F, t72: F) -> (F, F, F, F, F) {
    let t8124 = t660 * t755 * t186;
    let t8126 = F::cast_from(0.56968947174242584612e-3_f64) * t730 * t8124;
    let t8130 = t339 * t795 * t2376;
    let t8131 = t8130 * t803;
    let t8160 = F::new(1.0) / t237 / t207;
    let t8162 = t235 * t8160 * t72;
    (t8124, t8126, t8130, t8131, t8162)
}
