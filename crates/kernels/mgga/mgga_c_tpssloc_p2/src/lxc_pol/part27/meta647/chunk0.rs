//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2230/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2230<F: Float>(t25608: F, t6743: F, t1948: F, t6733: F, t23631: F, t61066: F, t974: F, t12652: F, t14586: F, t14595: F, t23323: F, t23327: F, t23609: F, t23657: F, t23673: F, t25502: F, t25510: F, t25511: F, t25512: F, t25523: F, t6797: F, t6799: F, t6800: F, t6801: F, t7603: F, t7615: F, t82539: F, t82555: F, t82643: F, t82657: F) -> (F, F) {
    let t89002 = t25608 * t6743;
    let t89019 = t6733 * t1948;
    let t89033 = t23631 * t974 * t61066;
    let t89042 = -F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t89002 * t6801 - F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t25510 * t25511 * t12652 + F::cast_from(0.54831135561607547884e-2_f64) * t82539 + F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t6799 * t14595 * t6800 + F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t6799 * t14586 * t6800 + F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t89019 * t25512 - F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t23657 * t25502 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t82643 * t7603 - F::cast_from(0.18277045187202515961e-2_f64) * t82555 + F::cast_from(0.80418998823691070228e-1_f64) * t23323 * t7615 - F::cast_from(0.54831135561607547884e-2_f64) * t89033 * t82657 - F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t25523 * t23609 - F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t25523 * t23673;
    (t89019, t89042)
}
