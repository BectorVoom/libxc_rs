//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 814/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk814<F: Float>(t698: F, t976: F, t979: F, t973: F, t135: F, t2978: F, t4509: F, t984: F, t2770: F, t343: F, t2775: F, t2769: F, t40: F, t986: F, t241: F, t625: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10224 = t698 * t976;
    let t10225 = t10224 * t979;
    let t10226 = t973 * t10225;
    let t10231 = t135 * t2978;
    let t10235 = t4509 * t984;
    let t10236 = t343 * t2770;
    let t10254 = t343 * t2775;
    let t10276 = t2769 * t40;
    let t10277 = 1.0 / t10276;
    let t10286 = t698 * t986;
    let t10287 = t973 * t10286;
    let t10292 = t625 * t241;
    (t10224, t10226, t10231, t10235, t10236, t10254, t10277, t10287, t10292)
}
