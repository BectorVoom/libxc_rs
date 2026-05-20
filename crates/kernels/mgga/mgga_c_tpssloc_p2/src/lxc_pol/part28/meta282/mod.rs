//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta282 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1178;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1179;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta282<F: Float>(t685: F, t9694: F, t120: F, t781: F, t118: F, t123: F, t116: F, t16: F, t2397: F, t9691: F, t693: F, t119: F, t133: F, t625: F, t9689: F, t9692: F, t739: F, t746: F, t761: F, t172: F, t2448: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t9695, t9697, t9698, t9702, t9704, t9706, t9709) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1178::<F>(t685, t9694, t120, t781, t118, t123, t116, t16, t2397, t9691, t693, t119, t133, t625);
        let (t9711, t9713, t9715, t9716) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1179::<F>(t9689, t9692, t9695, t9698, t9702, t9704, t9706, t9709, t739, t746, t761, t172, t2448);
    (t9695, t9697, t9698, t9702, t9704, t9706, t9709, t9711, t9713, t9715, t9716)
}
