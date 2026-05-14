//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 837/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk837<F: Float>(t10024: F, t238: F, t154: F, t9569: F, t222: F, t2606: F, t9573: F, t119: F, t210: F, t9458: F, t805: F, t9541: F, t2563: F, t2610: F, t9516: F, t10009: F, t10012: F, t10014: F, t10017: F, t249: F, t2643: F, t787: F, t9559: F) -> (F, F, F, F) {
    let t10026 = 595.0 / 10368.0 * t238 * t10024;
    let t10027 = t9569 * t154;
    let t10029 = 455.0 / 1296.0 * t10027 * t222;
    let t10030 = t9573 * t2606;
    let t10033 = t210 * t119 * t9458;
    let t10036 = t9541 * t805;
    let t10038 = t2563 * t2610;
    let t10041 = t210 * t119 * t9516;
    let t10044 = t2643 * t10009 / 256.0 - 7.0 / 1536.0 * t10012 + 119.0 / 4608.0 * t10014 + t10017 * t249 / 3072.0 - t10026 - t10029 - 7.0 / 16.0 * t10030 - t9559 * t10033 / 4.0 - 35.0 / 72.0 * t10036 + 7.0 / 48.0 * t10038 - t787 * t10041 / 48.0;
    (t10027, t10033, t10041, t10044)
}
