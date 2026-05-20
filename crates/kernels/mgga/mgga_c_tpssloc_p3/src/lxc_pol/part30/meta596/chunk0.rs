//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1978/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1978<F: Float>(t1336: F, t22759: F, t835: F, t22760: F, t3777: F, t12248: F, t6604: F, t22716: F, t6983: F, t22723: F, t268: F, t534: F) -> (F, F, F, F, F) {
    let t80997 = t1336 * t22759 * t835;
    let t81000 = t3777 * t22760;
    let t81027 = t6604 * t12248;
    let t81039 = t22716 * t6983;
    let t81046 = t22723 * t534 * t268;
    (t80997, t81000, t81027, t81039, t81046)
}
