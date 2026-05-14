//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 730/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk730<F: Float>(t219: F, t4747: F, t2357: F, t4706: F, t4701: F, t778: F, t1373: F, t1375: F, t222: F, t224: F) -> (F, F, F, F) {
    let t4748 = t4747 * t219;
    let t4752 = t2357 * t4706;
    let t4755 = t778 * t4701;
    let t4758 = 6.0 * t1373 * t1375 - 12.0 * t222 * t4752 + 3.0 * t222 * t4755 - t224 * t4748;
    (t4748, t4752, t4755, t4758)
}
