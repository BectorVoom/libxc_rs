//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 430/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk430<F: Float>(t3127: F, t363: F, t3037: F, t3033: F, t360: F, t3040: F, t1021: F, t248: F, t135: F, t999: F, t973: F, t2250: F, t998: F, t974: F, t2770: F, t2978: F) -> (F, F, F, F, F, F, F) {
    let t3128 = t3127 * t363;
    let t3129 = t3128 * t3037;
    let t3130 = t3033 * t3129;
    let t3131 = t360 * t360;
    let t3132 = t3040 * t3131;
    let t3134 = t248 * t1021 * t3132;
    let t3139 = t135 * t999;
    let t3140 = t973 * t3139;
    let t3142 = t998 * t2250;
    let t3143 = t974 * t3142;
    let t3146 = t2978 * t2770;
    (t3130, t3131, t3134, t3139, t3140, t3143, t3146)
}
