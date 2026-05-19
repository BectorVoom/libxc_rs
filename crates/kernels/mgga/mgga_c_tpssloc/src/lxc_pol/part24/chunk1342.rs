//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1342/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1342<F: Float>(t23229: F, t81715: F, t225: F, t23226: F, t23228: F, t23272: F, t81651: F, t23204: F, t23218: F, t6562: F, t23171: F, t6572: F) -> (F, F, F, F, F) {
    let t82069 = t81715 * t23229;
    let t82070 = F::cast_from(0.98696044010893586188e-1_f64) * t82069;
    let t82071 = t23226 * t225;
    let t82074 = t23228 * t225;
    let t82076 = t81651 * t82074 * t23272;
    let t82079 = t6562 * t23204 * t23218;
    let t82082 = t23171 * t23228 * t6572;
    (t82070, t82071, t82076, t82079, t82082)
}
