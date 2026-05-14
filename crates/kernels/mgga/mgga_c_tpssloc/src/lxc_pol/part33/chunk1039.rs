//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1039/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1039<F: Float>(t1409: F, t1933: F, t1937: F, t1597: F, t40: F, t23479: F, t1015: F, t7582: F, t23472: F, t343: F, t23562: F, t23509: F, t3: F, t23470: F, t3030: F, t1615: F, t3128: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t25628 = t1933 * t1409;
    let t25629 = t25628 * t1937;
    let t25637 = t40 * t1597;
    let t25638 = t1933 * t25637;
    let t25639 = t25638 * t23479;
    let t25641 = t1015 * t7582;
    let t25642 = t23472 * t25641;
    let t25644 = t25637 * t343;
    let t25645 = t23562 * t25644;
    let t25650 = t23509 * t3;
    let t25651 = t23470 * t3030;
    let t25652 = t25650 * t25651;
    let t25653 = t3128 * t1615;
    (t25629, t25639, t25641, t25642, t25644, t25645, t25650, t25651, t25652, t25653)
}
