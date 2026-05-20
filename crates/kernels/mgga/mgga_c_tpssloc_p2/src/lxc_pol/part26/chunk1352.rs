//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1352/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1352<F: Float>(t1235: F, t24594: F, t24705: F, t7327: F, t1176: F, t1184: F, t24847: F, t974: F, t1009: F, t460: F, t27495: F, t15702: F, t7329: F) -> (F, F, F, F) {
    let t85807 = t24594 * t1235;
    let t85814 = t24705 * t7327;
    let t85820 = t24847 * t974 * t1176 * t1184;
    let t85821 = t460 * t1009;
    let t85822 = t85821 * t27495;
    let t85824 = t85822 * t7329 * t15702;
    (t85807, t85814, t85820, t85824)
}
