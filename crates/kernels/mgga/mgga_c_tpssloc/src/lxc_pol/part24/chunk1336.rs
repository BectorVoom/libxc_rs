//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1336/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1336<F: Float>(t81778: F, t81845: F, t81905: F, t81974: F, t23034: F, t6546: F, t23037: F, t131: F, t845: F, t1878: F, t209: F, t6637: F, t6638: F, t9458: F) -> (F, F, F, F, F) {
    let t81976 = t81778 + t81845 + t81905 + t81974;
    let t81979 = t6546 * t23034;
    let t81980 = t81979 * t23037;
    let t81982 = t845 * t131;
    let t81984 = t1878 * t81982 * t209;
    let t81987 = t81984 * t6637 * t6638 * t9458;
    (t81976, t81979, t81980, t81984, t81987)
}
