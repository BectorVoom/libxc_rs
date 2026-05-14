//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1287/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1287<F: Float>(t63973: F, t10767: F, t5552: F, t17974: F, t3685: F, t10664: F, t5559: F, t10669: F, t10662: F, t19696: F, t215: F, t10667: F, t19695: F, t19697: F, t5543: F, t136: F, t1693: F, t799: F) -> (F, F, F, F, F, F, F, F, F) {
    let t63974 = 7.0 / 1152.0 * t63973;
    let t63975 = t5552 * t10767;
    let t63977 = t17974 * t3685;
    let t63978 = 35.0 / 288.0 * t63977;
    let t63979 = t5559 * t10664;
    let t63981 = t5559 * t10669;
    let t63984 = t19696 * t215 * t10662;
    let t63987 = t19696 * t215 * t10667;
    let t63990 = t5543 * t19695 * t19697;
    let t63991 = 7.0 / 24.0 * t63990;
    let t63993 = t1693 * t799 * t136;
    (t63974, t63975, t63978, t63979, t63981, t63984, t63987, t63991, t63993)
}
