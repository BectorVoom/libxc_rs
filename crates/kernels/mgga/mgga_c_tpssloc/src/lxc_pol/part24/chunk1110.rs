//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1110/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1110<F: Float>(t6796: F, t995: F, t6802: F, t614: F, t6794: F, t131: F, t350: F, t3196: F, t6800: F, t6799: F, t23602: F, t3127: F, t1011: F, t3131: F, t3187: F, t1049: F, t362: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t23665 = t6796 * t995;
    let t23666 = t23665 * t6802;
    let t23668 = t614 * t6794;
    let t23669 = t23668 * t131;
    let t23670 = t23669 * t350;
    let t23673 = t3196 * t6800;
    let t23674 = t6799 * t23673;
    let t23677 = t23602 * t3127;
    let t23678 = t1011 * t3131;
    let t23679 = t3187 * t23678;
    let t23680 = t23677 * t23679;
    let t23685 = t362 * t1049;
    (t23665, t23666, t23668, t23669, t23670, t23673, t23674, t23677, t23678, t23679, t23680, t23685)
}
