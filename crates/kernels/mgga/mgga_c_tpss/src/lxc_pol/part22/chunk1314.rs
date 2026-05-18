//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1314/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1314<F: Float>(t10767: F, t5552: F, t17974: F, t3685: F, t10664: F, t5559: F, t10669: F, t10662: F, t19696: F, t215: F, t10667: F, t19695: F, t19697: F, t5543: F) -> (F, F, F, F, F, F, F) {
    let t63975 = t5552 * t10767;
    let t63977 = t17974 * t3685;
    let t63979 = t5559 * t10664;
    let t63981 = t5559 * t10669;
    let t63984 = t19696 * t215 * t10662;
    let t63987 = t19696 * t215 * t10667;
    let t63990 = t5543 * t19695 * t19697;
    (t63975, t63977, t63979, t63981, t63984, t63987, t63990)
}
