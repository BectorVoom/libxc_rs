//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 830/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk830<F: Float>(t259: F, t379: F, t207: F, t5585: F, t1692: F, t1713: F, t198: F, t2439: F, t5590: F, t750: F, t821: F, t823: F, t330: F, t4023: F, t5648: F, t5652: F, t993: F, t995: F) -> (F, F) {
    let t380 = t259 < t379;
    let t5659 = t207 * t5585;
    let t5664 = -t1692 * t5590 * t821 + 3.0 * t1713 * t2439 * t750 + t198 * t5659 * t823;
    let t5665 = piecewise3(t380, t198 * t330 * t5648 * t995 - t4023 * t5652 * t993, t5664);
    (t5664, t5665)
}
