//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1137/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1137<F: Float>(t22987: F, t25038: F, t25248: F, t2553: F, t23005: F, t6579: F, t2631: F, t852: F, t1888: F, t232: F, t6646: F, t23181: F) -> (F, F, F, F, F) {
    let t81695 = t25038 * t25248 * t22987 * t2553;
    let t81697 = t6579 * t23005;
    let t81699 = t852 * t2631;
    let t81702 = t1888 * t6646 * t81699 * t232;
    let t81704 = t6579 * t23181;
    (t81695, t81697, t81699, t81702, t81704)
}
