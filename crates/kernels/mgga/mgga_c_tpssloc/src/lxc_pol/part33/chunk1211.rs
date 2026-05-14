//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1211/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1211<F: Float>(t1935: F, t1941: F, t21456: F, t21482: F, t21546: F, t25645: F, t28566: F, t28578: F, t28587: F, t343: F, t378: F, t6717: F, t6734: F, t7574: F, t7583: F, t83080: F, t88372: F, t99662: F, t99667: F, t99671: F, t99707: F) -> (F,) {
    let t106307 = 7.0 / 648.0 * t6717 * t21546 + t99707 / 1152.0 - 0.30279567070605293142e-3 * t7574 * t28566 - 0.10093189023535097714e-3 * t1935 * t21456 * t343 * t6734 + t21482 * t1941 * t378 / 1536.0 - 0.60559134141210586284e-3 * t99662 * t7583 - 0.30279567070605293142e-3 * t99667 * t7583 - 0.30279567070605293142e-3 * t25645 * t28587 - 0.60559134141210586284e-3 * t88372 * t28578 + t83080 - 0.30279567070605293142e-3 * t99671 * t7583;
    (t106307,)
}
