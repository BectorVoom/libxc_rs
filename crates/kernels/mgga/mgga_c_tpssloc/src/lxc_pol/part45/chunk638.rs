//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 638/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk638<F: Float>(t22511: F, t67: F, t1864: F, t6505: F, t6509: F, t2235: F, t608: F, t33: F, t6504: F, t2240: F, t641: F, t645: F, t72: F, t2307: F, t79: F, t2244: F, t605: F) -> (F, F, F, F, F, F, F) {
    let t22512 = t22511 * t67;
    let t22513 = t22512 * t1864;
    let t22516 = t6505 * t6509;
    let t22519 = t2235 * t608;
    let t22522 = t33 * t6504;
    let t22523 = t2240 * t22522;
    let t22527 = t72 * t641 * t645;
    let t22530 = t79 * t2307;
    let t22531 = t72 * t22530;
    let t22534 = t605 * t2244;
    (t22513, t22516, t22519, t22523, t22527, t22531, t22534)
}
