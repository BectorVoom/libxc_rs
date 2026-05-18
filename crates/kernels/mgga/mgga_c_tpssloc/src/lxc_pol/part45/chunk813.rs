//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 813/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk813<F: Float>(t2094: F, t531: F, t22596: F, t7025: F, t9239: F, t33: F, t625: F, t2240: F, t6492: F, t2031: F, t22550: F, t6495: F, t7032: F) -> (F, F, F, F, F, F) {
    let t23957 = t531 * t2094;
    let t23958 = t23957 * t22596;
    let t23963 = t9239 * t7025;
    let t23966 = t33 * t625;
    let t23967 = t2240 * t23966;
    let t23968 = t23967 * t6492;
    let t23970 = t2031 * t22550;
    let t23973 = t6495 * t7032;
    (t23958, t23963, t23966, t23968, t23970, t23973)
}
