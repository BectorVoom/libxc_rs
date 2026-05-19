//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 41 (v4rho3tau_5) CSE chunk 1003/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part41_v4rho3tau_5_chunk1003<F: Float>(t225: F, t4943: F, t1720: F, t3030: F, t3609: F, t1009: F, t4940: F, t1243: F, t14704: F, t14710: F, t14720: F, t14781: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14980 = t4943 * t225;
    let t15026 = t1720 * t3030;
    let t15027 = t15026 * t3609;
    let t15031 = t4940 * t1009;
    let t15032 = t15031 * t1243;
    let t15072 = F::cast_from(0.34431666666666666666e0_f64) * t14704;
    let t15074 = F::cast_from(0.13892666666666666667e0_f64) * t14710;
    let t15083 = F::cast_from(0.22954444444444444444e0_f64) * t14720;
    let t15094 = F::cast_from(0.27785333333333333334e0_f64) * t14781;
    (t14980, t15026, t15027, t15031, t15032, t15072, t15074, t15083, t15094)
}
