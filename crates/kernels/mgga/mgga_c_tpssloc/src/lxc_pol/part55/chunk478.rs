//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 478/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk478<F: Float>(t448: F, t1143: F, t300: F, t457: F, t697: F, t461: F) -> (F, F, F, F) {
    let t3402 = t448 * t448;
    let t3403 = 1.0 / t3402;
    let t3411 = t300 * t1143;
    let t3426 = t697 * t457;
    let t3427 = t3426 * t461;
    (t3403, t3411, t3426, t3427)
}
