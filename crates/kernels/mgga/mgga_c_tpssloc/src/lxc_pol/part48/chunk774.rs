//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 774/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk774<F: Float>(t2040: F, t31537: F, t6534: F, t89: F, t7050: F, t8526: F, t6535: F, t7042: F, t1377: F, t2091: F, t1307: F, t22635: F, t22633: F, t2015: F, t7213: F, t3887: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t31539 = 2.0 * t31537 * t2040;
    let t31540 = t89 * t6534;
    let t31542 = 2.0 * t31540 * t2040;
    let t31544 = 2.0 * t8526 * t7050;
    let t31548 = 2.0 * t7042 * t6535;
    let t31549 = t1377 * t2091;
    let t31550 = t31549 * t1307;
    let t31551 = t22635 * t31550;
    let t31552 = t22633 * t31551;
    let t31554 = t7213 * t2015;
    let t31555 = t3887 * t31554;
    (t31539, t31540, t31542, t31544, t31548, t31549, t31550, t31551, t31552, t31555)
}
