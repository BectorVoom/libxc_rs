//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1700/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1700<F: Float>(t5234: F, t6951: F, t1369: F, t1831: F, t22788: F, t5314: F, t6952: F, t1811: F, t22797: F, t22804: F, t7709: F, t1361: F, t1799: F, t22690: F) -> (F, F, F, F, F, F, F) {
    let t26257 = t5234 * t6951;
    let t26258 = t26257 * t1369;
    let t26260 = t22788 * t1831;
    let t26262 = t6952 * t5314;
    let t26266 = t22797 * t1811;
    let t26268 = t22804 * t7709;
    let t26271 = t22690 * t1361 * t1799;
    (t26257, t26258, t26260, t26262, t26266, t26268, t26271)
}
