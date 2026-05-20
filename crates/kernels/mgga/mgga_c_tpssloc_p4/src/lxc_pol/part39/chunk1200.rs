//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1200/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1200<F: Float>(t11211: F, t11213: F, t11314: F, t11317: F, t14702: F, t14708: F, t14713: F, t14759: F, t14779: F, t14784: F, t14787: F, t14790: F, t14793: F, t14796: F, t14799: F, t14802: F, t14805: F, t15072: F, t15074: F, t15091: F, t15094: F, t15115: F) -> F {
    let t15117 = -t11314 - t11317 + F::cast_from(0.22954444444444444444e0_f64) * t14702 - t15072 + F::new(0.516475e0) * t14708 - t15074 + F::new(0.104195e0) * t14713 + F::new(0.3529725e1) * t14759 + F::cast_from(0.23154444444444444444e0_f64) * t11211 + F::cast_from(0.23154444444444444444e-1_f64) * t11213 + t15091 + F::cast_from(0.46308888888888888889e-1_f64) * t14779 - t15094 - F::cast_from(0.69463333333333333334e-1_f64) * t14784 - F::cast_from(0.34731666666666666667e-1_f64) * t14787 - F::new(0.20839e0) * t14790 + F::new(0.41678e0) * t14793 + F::new(0.20839e0) * t14796 + F::new(0.62517e0) * t14799 + F::cast_from(0.264729375e1_f64) * t14802 - F::cast_from(0.157790625e0_f64) * t14805 + t15115;
    t15117
}
