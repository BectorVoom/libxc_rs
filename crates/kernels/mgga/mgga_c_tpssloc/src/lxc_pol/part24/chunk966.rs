//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 966/1291 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk966<F: Float>(t1174: F, t11841: F, t1196: F, t9258: F, t974: F, t1176: F, t3242: F, t9288: F, t11638: F, t475: F, t1214: F, t248: F, t11616: F, t68: F, t484: F, t10913: F, t4972: F) -> (F, F, F, F, F, F) {
    let t11842 = t1174 * t11841;
    let t11844 = t1196 * t9258;
    let t11845 = t974 * t11844;
    let t11848 = t1176 * t3242;
    let t11849 = t11848 * t9288;
    let t11850 = t974 * t11849;
    let t11853 = t11638 * t475;
    let t11855 = t248 * t1214 * t11853;
    let t11858 = t11616 * t68;
    let t11859 = t11858 * t484;
    let t11862 = t4972 * t10913;
    (t11842, t11845, t11850, t11855, t11859, t11862)
}
