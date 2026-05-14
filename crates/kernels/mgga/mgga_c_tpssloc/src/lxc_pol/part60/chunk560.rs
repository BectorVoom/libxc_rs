//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 560/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk560<F: Float>(t559: F, t7715: F, t1827: F, t6945: F, t1831: F, t6952: F, t6915: F, t6922: F, t6935: F, t6949: F, t7706: F, t7710: F, t7713: F) -> (F, F, F, F) {
    let t7716 = t7715 * t559;
    let t7718 = t6945 * t1827;
    let t7720 = t6952 * t1831;
    let t7722 = -t6915 - t7706 / 48.0 - t6922 - 0.12111826828242117256e-2 * t7710 - t6935 - 0.20186378047070195427e-3 * t7713 + t7716 / 1536.0 - t7718 / 1536.0 - t6949 - t7720 / 384.0;
    (t7716, t7718, t7720, t7722)
}
