//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1304/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1304<F: Float>(t3418: F, t3432: F, t1317: F, t3482: F, t77: F, t1313: F, t3486: F, t21115: F, t619: F, t6076: F, t1679: F, t4570: F) -> (F, F, F, F, F, F) {
    let t69114 = t3418 * t3432;
    let t69135 = t77 * t3482 * t1317;
    let t69139 = t77 * t1313 * t3486;
    let t69143 = t77 * t21115 * t619;
    let t69147 = t77 * t6076 * t3486;
    let t69152 = t1679 * t4570;
    (t69114, t69135, t69139, t69143, t69147, t69152)
}
