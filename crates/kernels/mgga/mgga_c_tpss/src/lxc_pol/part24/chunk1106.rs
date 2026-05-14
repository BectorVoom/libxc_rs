//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1106/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1106<F: Float>(t2957: F, t5145: F, t1061: F, t4142: F, t4146: F, t5129: F, t9467: F, t1080: F, t5162: F, t1543: F, t4180: F, t5181: F, t5178: F, t3001: F, t5177: F, t4184: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15736 = t5145 * t2957;
    let t15737 = t15736 * t1061;
    let t15740 = t4146 * t4142;
    let t15743 = t5129 * t9467;
    let t15744 = t15743 * t1061;
    let t15751 = t5162 * t1080;
    let t15754 = t1543 * t4180;
    let t15757 = t5181 * t1080;
    let t15760 = t5178 * t1080;
    let t15763 = t5177 * t3001;
    let t15764 = t15763 * t1080;
    let t15767 = t4184 * t4180;
    (t15737, t15740, t15744, t15751, t15754, t15757, t15760, t15764, t15767)
}
