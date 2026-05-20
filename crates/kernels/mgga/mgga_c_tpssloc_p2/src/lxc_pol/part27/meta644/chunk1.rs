//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2199/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2199<F: Float>(t1409: F, t14165: F, t23327: F, t23329: F, t23402: F, t25430: F, t25442: F, t25443: F, t25750: F, t25815: F, t3175: F, t6691: F, t7557: F, t82382: F, t82402: F, t82417: F, t82502: F, t88058: F, t88069: F, t88075: F, t88076: F, t88083: F, t88089: F, t88096: F) -> F {
    let t88097 = F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t82502 * t25750 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88058 * t6691 + F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t25442 * t23402 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t82417 * t25815 - t88069 + F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t23329 * t25430 * t14165 - t88075 + F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23329 * t88076 * t1409 * t3175 - t88083 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t82417 * t25750 + F::cast_from(0.14621636149762012769e-1_f64) * t82402 * t25443 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88089 * t6691 - F::cast_from(0.80418998823691070228e-1_f64) * t82382 * t7557 + t88096;
    t88097
}
