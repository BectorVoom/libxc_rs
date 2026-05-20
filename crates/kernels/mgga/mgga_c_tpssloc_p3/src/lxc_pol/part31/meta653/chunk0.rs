//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1932/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1932<F: Float>(t16932: F, t25084: F, t16937: F, t16907: F, t23146: F, t17009: F, t17013: F, t25111: F, t7496: F, t87447: F, t22690: F, t23122: F, t5544: F, t841: F) -> (F, F, F, F, F, F, F) {
    let t98631 = t25084 * t16932;
    let t98633 = t25084 * t16937;
    let t98635 = t23146 * t16907;
    let t98637 = t23146 * t17009;
    let t98639 = t23146 * t17013;
    let t98642 = t87447 * t7496 * t25111;
    let t98647 = t23122 * t22690 * t841 * t5544;
    (t98631, t98633, t98635, t98637, t98639, t98642, t98647)
}
