//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1106/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1106<F: Float>(t17635: F, t4588: F, t4582: F, t1023: F, t5681: F, t3071: F, t248: F, t3101: F, t5878: F, t3039: F, t3051: F, t5685: F) -> (F, F, F, F) {
    let t17642 = t4588 * t17635;
    let t17643 = t4582 * t17642;
    let t17648 = t5681 * t1023;
    let t17649 = t3071 * t17648;
    let t17655 = t248 * t3101 * t5878;
    let t17656 = t3039 * t17655;
    let t17659 = t248 * t3051 * t5685;
    (t17643, t17649, t17656, t17659)
}
