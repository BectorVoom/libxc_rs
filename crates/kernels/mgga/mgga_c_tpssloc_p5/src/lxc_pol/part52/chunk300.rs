//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 300/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk300<F: Float>(t1147: F, t1155: F, t1156: F, t1164: F, t134: F, t457: F, t461: F) -> (F, F, F, F) {
    let t1166 = t1147 * t1155 * t1156;
    let t1168 = F::cast_from(0.5848223622634646207e0_f64) * t1164 * t1166;
    let t1169 = t134 * t457;
    let t1170 = t1169 * t461;
    (t1166, t1168, t1169, t1170)
}
