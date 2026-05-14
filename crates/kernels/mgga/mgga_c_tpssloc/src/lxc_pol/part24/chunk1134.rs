//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1134/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1134<F: Float>(t1404: F, t7002: F, t2029: F, t3931: F, t2022: F, t3946: F, t1983: F, t23857: F, t6996: F, t22579: F, t6876: F, t26161: F, t26162: F, t55173: F, t24995: F, t53789: F, t8643: F) -> (F, F, F, F, F, F, F) {
    let t80599 = t7002 * t1404;
    let t80601 = t3931 * t2029;
    let t80605 = t2022 * t3946;
    let t80609 = 6.0 * t1983 * t6996 * t23857;
    let t80611 = 3.0 * t6876 * t22579;
    let t80614 = 6.0 * t26161 * t26162 * t55173;
    let t80617 = 18.0 * t24995 * t8643 * t53789;
    (t80599, t80601, t80605, t80609, t80611, t80614, t80617)
}
