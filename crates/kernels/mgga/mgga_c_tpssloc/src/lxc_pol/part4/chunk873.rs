//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 873/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk873<F: Float>(t14722: F, t14781: F, t14720: F, t14704: F, t225: F, t4947: F, t4943: F, t1720: F, t3030: F, t3609: F, t1009: F, t4940: F, t1243: F, t14710: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14886 = 0.39862222222222222222e0 * t14722;
    let t14890 = 0.21908444444444444444e0 * t14781;
    let t14922 = 0.41203703703703703704e-2 * t14720;
    let t14923 = 0.12361111111111111111e-1 * t14722;
    let t14924 = 0.61805555555555555556e-2 * t14704;
    let t14946 = 0.23744444444444444444e-1 * t14722;
    let t14947 = 0.11872222222222222222e-1 * t14704;
    let t14972 = t4947 * t225;
    let t14980 = t4943 * t225;
    let t15026 = t1720 * t3030;
    let t15027 = t15026 * t3609;
    let t15031 = t4940 * t1009;
    let t15032 = t15031 * t1243;
    let t15072 = 0.34431666666666666666e0 * t14704;
    let t15074 = 0.13892666666666666667e0 * t14710;
    let t15083 = 0.22954444444444444444e0 * t14720;
    (t14886, t14890, t14922, t14923, t14924, t14946, t14947, t14972, t14980, t15026, t15027, t15031, t15032, t15072, t15074, t15083)
}
