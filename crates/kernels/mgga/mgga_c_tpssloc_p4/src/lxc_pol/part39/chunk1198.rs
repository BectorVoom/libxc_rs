//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1198/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1198<F: Float>(t15067: F, t3265: F, t11275: F, t14704: F, t14710: F, t14720: F, t11215: F, t11217: F, t14722: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t14766: F) -> (F, F, F, F) {
    let t15068 = t15067 * t3265;
    let t15070 = F::cast_from(0.51726012919273400301e3_f64) * t11275 * t15068;
    let t15072 = F::cast_from(0.34431666666666666666e0_f64) * t14704;
    let t15074 = F::cast_from(0.13892666666666666667e0_f64) * t14710;
    let t15083 = F::cast_from(0.22954444444444444444e0_f64) * t14720;
    let t15091 = -F::cast_from(0.13892666666666666667e0_f64) * t11215 - F::cast_from(0.69463333333333333333e-1_f64) * t11217 + F::cast_from(0.11577222222222222222e0_f64) * t14766 + t15083 - F::cast_from(0.68863333333333333334e0_f64) * t14738 - F::cast_from(0.34431666666666666667e0_f64) * t14742 - F::new(0.20659e1) * t14733 + F::new(0.20659e1) * t14751 + F::new(0.103295e1) * t14755 + F::new(0.309885e1) * t14746 - F::cast_from(0.68863333333333333333e0_f64) * t14722;
    (t15070, t15072, t15074, t15091)
}
