//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 990/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk990<F: Float>(t10698: F, t682: F, t2345: F, t3557: F, t10557: F, t10559: F, t10561: F, t10566: F, t10568: F, t10686: F, t10688: F, t10692: F, t10693: F, t10694: F, t10697: F, t8126: F, t8222: F) -> (F, F, F) {
    let t10700 = 8.0 * t10698 * t682;
    let t10701 = t3557 * t2345;
    let t10702 = 0.11696447245269292414e1 * t10701;
    let t10703 = t10557 - t8126 - t10559 - t10561 + t10566 + t10568 - t10686 + t10688 + t10692 - t10693 + t10694 + t10697 + t10700 + t10702 + t8222;
    (t10700, t10702, t10703)
}
