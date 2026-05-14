//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 791/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk791<F: Float>(t1805: F, t818: F, t5572: F, t226: F, t5577: F, t782: F, t1708: F, t228: F, t5831: F, t1707: F, t1809: F, t253: F, t5568: F, t5571: F, t5832: F, t5834: F, t819: F) -> (F, F, F, F) {
    let t5837 = t1805 * t818;
    let t5838 = t5572 * t5837;
    let t5843 = t5577 * t1805 * t782 * t226;
    let t5846 = t1708 * t228 * t5831;
    let t5848 = -t1707 * t5846 - t1809 * t5568 + t253 * t5832 + 2.0 * t5571 * t5838 + t5571 * t5843 - t5834 * t819;
    (t5838, t5843, t5846, t5848)
}
