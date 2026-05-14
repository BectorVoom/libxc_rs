//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 3 (v3rho3_1) CSE chunk 608/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part3_v3rho3_1_chunk608<F: Float>(t3040: F, t3131: F, t1021: F, t248: F, t135: F, t999: F, t973: F, t2250: F, t998: F, t974: F, t2770: F, t2978: F, t2244: F, t2775: F, t976: F, t1005: F, t1036: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3132 = t3040 * t3131;
    let t3134 = t248 * t1021 * t3132;
    let t3139 = t135 * t999;
    let t3140 = t973 * t3139;
    let t3142 = t998 * t2250;
    let t3143 = t974 * t3142;
    let t3146 = t2978 * t2770;
    let t3147 = t3146 * t2244;
    let t3148 = t974 * t3147;
    let t3151 = t976 * t2775;
    let t3152 = t3151 * t2244;
    let t3153 = t974 * t3152;
    let t3156 = t1005 * t1036;
    (t3132, t3134, t3139, t3140, t3142, t3143, t3147, t3148, t3152, t3153, t3156)
}
