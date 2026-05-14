//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1037/1116 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1037<F: Float>(t15054: F, t3264: F, t3265: F, t4785: F, t11190: F, t3315: F, t4781: F, t1117: F, t3313: F, t3307: F, t11277: F, t1670: F, t11275: F, t14704: F, t14710: F, t14720: F) -> (F, F, F, F, F, F, F, F) {
    let t15056 = 2.0 * t3264 * t15054;
    let t15057 = t4785 * t3265;
    let t15059 = 0.96491876992155210402e2 * t11190 * t15057;
    let t15060 = t4781 * t3315;
    let t15061 = t15060 * t1117;
    let t15063 = 0.32163958997385070134e2 * t3313 * t15061;
    let t15064 = t4785 * t3307;
    let t15066 = 0.16081979498692535067e2 * t3313 * t15064;
    let t15067 = t1670 * t11277;
    let t15068 = t15067 * t3265;
    let t15070 = 0.51726012919273400301e3 * t11275 * t15068;
    let t15072 = 0.34431666666666666666e0 * t14704;
    let t15074 = 0.13892666666666666667e0 * t14710;
    let t15083 = 0.22954444444444444444e0 * t14720;
    (t15056, t15059, t15063, t15066, t15070, t15072, t15074, t15083)
}
