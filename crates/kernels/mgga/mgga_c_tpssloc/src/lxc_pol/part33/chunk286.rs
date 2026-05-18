//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 286/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk286<F: Float>(t1086: F, t432: F, t427: F, t1111: F, t435: F) -> (F, F, F, F, F, F, F) {
    let t1122 = F::new(0.17123333333333333333e-1) * t1086;
    let t1127 = t432 * t432;
    let t1128 = F::new(1.0) / t1127;
    let t1129 = t427 * t1128;
    let t1131 = F::new(0.516475e0) * t1086;
    let t1134 = F::new(0.104195e0) * t1111;
    let t1137 = F::new(1.0) / t435;
    (t1122, t1127, t1128, t1129, t1131, t1134, t1137)
}
