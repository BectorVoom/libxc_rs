//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 837/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk837<F: Float>(t10213: F, t974: F, t2769: F, t632: F, t698: F, t976: F, t979: F, t973: F, t2970: F, t2999: F, t135: F, t2978: F, t2981: F, t4509: F, t984: F, t2770: F, t343: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10214 = t974 * t10213;
    let t10216 = 1.0 / t2769 / t632;
    let t10224 = t698 * t976;
    let t10225 = t10224 * t979;
    let t10226 = t973 * t10225;
    let t10228 = t2970 * t2999;
    let t10229 = t973 * t10228;
    let t10231 = t135 * t2978;
    let t10232 = t10231 * t2981;
    let t10233 = t973 * t10232;
    let t10235 = t4509 * t984;
    let t10236 = t343 * t2770;
    (t10214, t10216, t10224, t10226, t10229, t10231, t10233, t10235, t10236)
}
