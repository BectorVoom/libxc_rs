//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1871/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1871<F: Float>(t22704: F, t22705: F, t28167: F, t26331: F, t26421: F, t26446: F, t5187: F, t1992: F, t22897: F, t3792: F, t57607: F, t19745: F, t81027: F) -> (F, F, F, F) {
    let t96989 = t22704 * t22705 * t28167;
    let t96993 = t26331 * t26446 * t26421 * t5187;
    let t96997 = t1992 * t22897 * t57607 * t3792;
    let t97002 = t1992 * t81027 * t19745;
    (t96989, t96993, t96997, t97002)
}
