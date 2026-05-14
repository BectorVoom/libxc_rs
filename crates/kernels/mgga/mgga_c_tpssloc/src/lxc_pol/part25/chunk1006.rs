//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1006/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1006<F: Float>(t22674: F, t22934: F, t6897: F, t1307: F, t1377: F, t22633: F, t22635: F, t3911: F, t22935: F, t6883: F, t22667: F, t1987: F, t81144: F, t9537: F, t107: F, t835: F) -> (F, F, F, F, F, F) {
    let t81379 = t6897 * t22674 * t22934;
    let t81386 = t22633 * t22635 * t1377 * t3911 * t1307;
    let t81393 = t6883 * t22935;
    let t81395 = t6883 * t22667;
    let t81398 = t81144 * t9537 * t1987;
    let t81437 = t835 * t107;
    (t81379, t81386, t81393, t81395, t81398, t81437)
}
