//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 706/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk706<F: Float>(t2970: F, t979: F, t973: F, t135: F, t986: F, t271: F, t883: F) -> (F, F, F) {
    let t2971 = t2970 * t979;
    let t2972 = t973 * t2971;
    let t2974 = t135 * t986;
    let t2975 = t973 * t2974;
    let t2978 = F::new(1.0) / t271 / t883;
    (t2972, t2975, t2978)
}
