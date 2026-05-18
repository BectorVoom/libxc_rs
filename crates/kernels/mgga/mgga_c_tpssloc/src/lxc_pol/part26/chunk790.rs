//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 790/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk790<F: Float>(t31: F, t9258: F, t65: F, t2251: F, t628: F, t2283: F, t608: F, t36: F, t366: F, t41: F, t42: F, t2244: F, t607: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t9259 = t31 * t9258;
    let t9260 = t9259 * t65;
    let t9263 = t2251 * t628;
    let t9268 = t608 * t2283;
    let t9276 = F::new(1.0) / t36 / t366;
    let t9277 = sigma0 * t9276;
    let t9287 = F::new(1.0) / t42 / t41;
    let t9288 = t2244 * t607;
    (t9259, t9260, t9263, t9268, t9277, t9287, t9288)
}
