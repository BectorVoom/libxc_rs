//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 834/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk834<F: Float>(t2617: F, t2696: F, t849: F, t232: F, t2553: F, t2614: F, t838: F, t2693: F, t809: F, t597: F, t61: F, t241: F, t244: F, t248: F, t238: F, t154: F, t9569: F) -> (F, F, F, F, F, F, F, F) {
    let t9993 = t2617 * t2696;
    let t9994 = t9993 * t849;
    let t10007 = t232 * t2553;
    let t10012 = t2614 * t838;
    let t10014 = t809 * t2693;
    let t10021 = 1.0 / t61 / t597;
    let t10022 = t10021 * t241;
    let t10024 = t10022 * t244 * t248;
    let t10026 = 595.0 / 10368.0 * t238 * t10024;
    let t10027 = t9569 * t154;
    (t9993, t9994, t10007, t10012, t10014, t10022, t10026, t10027)
}
