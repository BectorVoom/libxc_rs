//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1019/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1019<F: Float>(t2094: F, t3701: F, t111: F, t2098: F, t192: F, t531: F, t1982: F, t25: F, t870: F, t7484: F, t794: F, t6562: F) -> (F, F, F, F, F, F) {
    let t24432 = t2094 * t3701;
    let t24465 = t2098 * t111;
    let t24994 = t192 * t531;
    let t24995 = t1982 * t24994;
    let t25014 = t870 * t25;
    let t25035 = t794 * t7484;
    let t25036 = t6562 * t25035;
    (t24432, t24465, t24995, t25014, t25035, t25036)
}
