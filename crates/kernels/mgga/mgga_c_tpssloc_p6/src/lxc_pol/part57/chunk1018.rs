//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 1018/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk1018<F: Float>(t127162: F, t26161: F, t26558: F, t28860: F, t8607: F, t19596: F, t1983: F, t8640: F, t1458: F, t33553: F, t652: F, t1873: F, t29197: F) -> (F, F, F, F, F) {
    let t128474 = F::new(4.0) * t26161 * t26558 * t127162;
    let t128475 = t8607 * t28860;
    let t128477 = t1983 * t8640 * t19596;
    let t128482 = F::new(4.0) * t652 * t33553 * t1458;
    let t128485 = F::new(2.0) * t652 * t29197 * t1873;
    (t128474, t128475, t128477, t128482, t128485)
}
