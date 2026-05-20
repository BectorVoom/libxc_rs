//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 363/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk363<F: Float>(t1086: F, t1111: F, t1092: F, t1103: F, t1108: F, t1115: F) -> (F, F, F) {
    let t1131 = F::new(0.516475e0) * t1086;
    let t1134 = F::new(0.104195e0) * t1111;
    let t1136 = F::new(0.3529725e1) * t1103 - t1131 + F::new(0.516475e0) * t1092 + F::new(0.6311625e0) * t1108 - t1134 + F::new(0.104195e0) * t1115;
    (t1131, t1134, t1136)
}
