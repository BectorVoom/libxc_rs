//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 878/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk878<F: Float>(t7025: F, t9239: F, t33: F, t625: F, t2240: F, t6492: F, t2031: F, t22550: F, t6495: F, t7032: F, t9231: F, t6486: F, t240: F, t67: F, t1864: F, t1860: F) -> (F, F, F, F, F, F, F, F) {
    let t23963 = t9239 * t7025;
    let t23966 = t33 * t625;
    let t23967 = t2240 * t23966;
    let t23968 = t23967 * t6492;
    let t23970 = t2031 * t22550;
    let t23973 = t6495 * t7032;
    let t23975 = t9231 * t7025;
    let t23978 = t6486 * t7032;
    let t23992 = t240 * t67;
    let t23993 = t23992 * t1864;
    let t23995 = 88.0 / 27.0 * t1860 * t23993;
    (t23963, t23967, t23968, t23970, t23973, t23975, t23978, t23995)
}
