//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 819/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk819<F: Float>(t9218: F, t2230: F, t594: F, t2229: F, t3: F) -> (F, F, F, F) {
    let t9219 = F::new(0.12804e4) * t9218;
    let t9220 = t594 * t2230;
    let t9221 = F::new(0.170856e4) * t9220;
    let t9222 = t2229 * t3;
    let t9223 = F::new(1.0) / t9222;
    (t9219, t9221, t9222, t9223)
}
