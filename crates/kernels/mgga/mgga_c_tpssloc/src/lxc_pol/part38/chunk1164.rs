//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1164/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1164<F: Float>(t29900: F, t8139: F, t64: F, t9365: F, t2332: F, t8129: F, t38: F, t96: F, t666: F, t659: F, t8138: F, t2358: F, t614: F, t656: F, t95: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t29901 = t29900 * t8139;
    let t29903 = t64 * t9365;
    let t29904 = t8129 * t2332;
    let t29907 = t38 * t96;
    let t29908 = t29907 * t666;
    let t29911 = t666 * t659;
    let t29912 = t8138 * t29911;
    let t29915 = t8129 * t2358;
    let t29919 = t656 * t614 * t96;
    let t29922 = t38 * t95;
    (t29901, t29903, t29904, t29907, t29908, t29911, t29912, t29915, t29919, t29922)
}
