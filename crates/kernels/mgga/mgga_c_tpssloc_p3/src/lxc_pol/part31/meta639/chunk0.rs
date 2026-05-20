//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1907/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1907<F: Float>(t22960: F, t67128: F, t5527: F, t606: F, t1408: F, t4303: F, t5664: F, t868: F, t86716: F, t776: F, t25373: F, t1530: F, t4119: F) -> (F, F, F, F, F, F, F, F) {
    let t97956 = t22960 * t67128;
    let t97985 = t606 * t5527;
    let t97990 = t1408 * t4303;
    let t97999 = t5664 * t868;
    let t98000 = t86716 * t97999;
    let t98003 = t5664 * t776;
    let t98004 = t25373 * t98003;
    let t98007 = t4119 * t1530;
    (t97956, t97985, t97990, t97999, t98000, t98003, t98004, t98007)
}
