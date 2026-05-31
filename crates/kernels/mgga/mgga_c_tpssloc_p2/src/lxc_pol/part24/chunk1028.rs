//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1028/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1028<F: Float>(t11714: F, t478: F, t10477: F, t483: F, t11713: F, t1215: F, t3507: F, t3508: F, t475: F, t1214: F, t248: F, t3503: F) -> (F, F, F, F, F, F, F) {
    let t11715 = F::cast_from(1.0_f64) / t11714;
    let t11716 = t11715 * t478;
    let t11717 = t483 * t10477;
    let t11718 = t11716 * t11717;
    let t11719 = t11713 * t11718;
    let t11720 = t3507 * t1215;
    let t11721 = t3508 * t475;
    let t11722 = t11720 * t11721;
    let t11724 = t248 * t1214 * t11722;
    let t11727 = t3503 * t11717;
    (t11715, t11717, t11719, t11720, t11721, t11724, t11727)
}
