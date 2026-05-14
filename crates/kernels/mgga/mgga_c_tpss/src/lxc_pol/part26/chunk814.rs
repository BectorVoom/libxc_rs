//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 814/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk814<F: Float>(t1702: F, t226: F, t782: F, t5577: F, t1708: F, t228: F, t5562: F, t1707: F, t1710: F, t253: F, t5563: F, t5565: F, t5568: F, t5571: F, t5574: F, t819: F) -> (F, F, F) {
    let t5579 = t1702 * t782 * t226;
    let t5580 = t5577 * t5579;
    let t5583 = t1708 * t228 * t5562;
    let t5585 = -t1707 * t5583 - t1710 * t5568 + t253 * t5563 - t5565 * t819 + 2.0 * t5571 * t5574 + t5571 * t5580;
    (t5580, t5583, t5585)
}
