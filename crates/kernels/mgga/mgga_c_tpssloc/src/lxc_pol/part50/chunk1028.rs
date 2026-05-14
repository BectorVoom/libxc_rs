//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1028/1149 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1028<F: Float>(t10143: F, t8365: F, t193: F, t201: F, t8369: F, t1054: F, t6815: F, t23384: F, t30862: F, t1921: F, t30789: F, t30908: F, t225: F, t30844: F, t30808: F, t1945: F, t6733: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t113117 = t8365 * t10143;
    let t113131 = t193 * t201 * t8365;
    let t113135 = t193 * t201 * t8369;
    let t113149 = t1054 * t6815;
    let t113177 = t23384 * t30862;
    let t113201 = t1921 * t113149;
    let t113207 = t23384 * t30789;
    let t113217 = t23384 * t30908;
    let t113219 = t30844 * t225;
    let t113231 = t30808 * t225;
    let t113236 = t6733 * t1945;
    (t113117, t113131, t113135, t113149, t113177, t113201, t113207, t113217, t113219, t113231, t113236)
}
