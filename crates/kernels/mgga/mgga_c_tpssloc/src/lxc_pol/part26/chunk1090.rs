//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1090/1236 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1090<F: Float>(t1983: F, t23857: F, t6996: F, t22579: F, t6876: F, t26161: F, t26162: F, t55173: F, t24995: F, t53789: F, t8643: F, t22948: F, t6999: F, t23831: F, t4034: F, t11968: F, t1873: F, t652: F) -> (F, F, F, F, F, F, F) {
    let t80609 = 6.0 * t1983 * t6996 * t23857;
    let t80611 = 3.0 * t6876 * t22579;
    let t80614 = 6.0 * t26161 * t26162 * t55173;
    let t80617 = 18.0 * t24995 * t8643 * t53789;
    let t80620 = 3.0 * t1983 * t22948 * t6999;
    let t80622 = 6.0 * t4034 * t23831;
    let t80625 = 2.0 * t652 * t11968 * t1873;
    (t80609, t80611, t80614, t80617, t80620, t80622, t80625)
}
