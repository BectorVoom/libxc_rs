//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 495/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk495<F: Float>(t1713: F, t30: F, t1692: F, t331: F, t43: F, t136: F, t347: F, sigma0: F) -> (F, F, F, F) {
    let t1714 = t1713 * t30;
    let t1716 = t1692 * t1714 / 2.0;
    let t1717 = t43 * t331;
    let t1718 = t1717 * t136;
    let t1721 = t347 * sigma0;
    (t1716, t1717, t1718, t1721)
}
