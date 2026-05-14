//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1202/1312 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1202<F: Float>(t24987: F, t8641: F, t120705: F, t22574: F, t24432: F, t31295: F, t7685: F, t19577: F, t36740: F, t115925: F, t25971: F, t8644: F, t101138: F, t26161: F, t31775: F, t1441: F, t6534: F) -> (F, F, F, F, F, F, F, F) {
    let t120888 = t24987 * t8641;
    let t120891 = 3.0 * t22574 * t24432 * t120705;
    let t120892 = t7685 * t31295;
    let t120896 = 3.0 * t22574 * t36740 * t19577;
    let t120899 = 3.0 * t115925 * t25971;
    let t120900 = t24987 * t8644;
    let t120907 = 2.0 * t26161 * t101138 * t31775;
    let t120908 = t1441 * t6534;
    (t120888, t120891, t120892, t120896, t120899, t120900, t120907, t120908)
}
