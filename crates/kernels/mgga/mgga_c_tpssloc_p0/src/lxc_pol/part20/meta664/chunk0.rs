//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2485/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2485<F: Float>(t14730: F, t9288: F, t1113: F, t136: F, t12606: F, t3242: F, t607: F, t3297: F, t123: F, t3240: F, t50857: F) -> (F, F, F, F, F) {
    let t50879 = t14730 * t9288;
    let t50881 = t136 * t1113 * t50879;
    let t50884 = t3242 * t12606 * t607;
    let t50886 = t136 * t3297 * t50884;
    let t50897 = t123 * t3240 * t50857;
    (t50879, t50881, t50884, t50886, t50897)
}
