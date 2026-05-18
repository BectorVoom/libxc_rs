//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1326/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1326<F: Float>(t16673: F, t6620: F, t23083: F, t28375: F, t28396: F, t81835: F, t23110: F, t23185: F, t28321: F, t23168: F, t28277: F, t28295: F, t6547: F) -> (F, F, F, F, F, F) {
    let t98832 = t16673 * t6620;
    let t98836 = t23083 * t28375;
    let t98838 = t81835 * t28396;
    let t98884 = t23185 * t23110 * t28321;
    let t98921 = t23168 * t28277;
    let t98923 = t6547 * t28295;
    (t98832, t98836, t98838, t98884, t98921, t98923)
}
