//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1277/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1277<F: Float>(t10456: F, t6106: F, t19327: F, t2056: F, t5511: F, t645: F, t19809: F, t60960: F, t1288: F, t2428: F, t1692: F, t1989: F, t5586: F, t2: F, t823: F, t555: F, t750: F) -> (F, F, F, F, F, F, F) {
    let t63751 = 4.0 * t10456 * t6106;
    let t63753 = 4.0 * t2056 * t19327;
    let t63756 = t5511 * t645;
    let t63766 = t60960 * t19809;
    let t63771 = t1288 * t2428;
    let t63782 = 2.0 * t1692 * t5586 * t1989;
    let t63783 = t823 * t2;
    let t63785 = t63783 * t555 * t750;
    (t63751, t63753, t63756, t63766, t63771, t63782, t63785)
}
