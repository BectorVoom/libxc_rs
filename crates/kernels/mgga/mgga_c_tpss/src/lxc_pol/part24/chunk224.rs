//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 224/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk224<F: Float>(t123: F, t173: F, t186: F, t651: F, t654: F, t679: F, t699: F, t706: F, t714: F, t721: F) -> (F,) {
    let t724 = 0.53237641966666666666e-3 * t123 * t651 * t173 + 1.0 * t699 * t706 - t654 - t679 + 0.18311447306006545054e-3 * t123 * t651 * t186 + 0.5848223622634646207e0 * t714 * t721;
    (t724,)
}
