//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2071/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2071<F: Float>(t1920: F, t23353: F, t968: F, t10164: F, t225: F, t23384: F, t23595: F, t23408: F, t1921: F, t6733: F, t3034: F, t336: F) -> (F, F, F, F, F, F) {
    let t82463 = t1920 * t968 * t23353;
    let t82481 = t225 * t10164;
    let t82490 = t23384 * t23595;
    let t82499 = t23408 * t225;
    let t82502 = t6733 * t1921;
    let t82510 = F::cast_from(1.0_f64) / t3034 / t336;
    (t82463, t82481, t82490, t82499, t82502, t82510)
}
