//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1034/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1034<F: Float>(t1597: F, t40: F, t1933: F, t23479: F, t1015: F, t7582: F, t23472: F, t343: F, t23562: F, t23509: F, t3: F, t23470: F, t3030: F) -> (F, F, F, F, F) {
    let t25637 = t40 * t1597;
    let t25638 = t1933 * t25637;
    let t25639 = t25638 * t23479;
    let t25641 = t1015 * t7582;
    let t25642 = t23472 * t25641;
    let t25644 = t25637 * t343;
    let t25645 = t23562 * t25644;
    let t25650 = t23509 * t3;
    let t25651 = t23470 * t3030;
    (t25639, t25642, t25645, t25650, t25651)
}
