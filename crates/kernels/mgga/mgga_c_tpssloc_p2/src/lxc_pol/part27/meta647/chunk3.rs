//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2233/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2233<F: Float>(t7604: F, t82573: F, t3961: F, t6746: F, t11046: F, t1409: F, t14213: F, t14571: F, t14630: F, t1629: F, t23327: F, t23511: F, t23613: F, t23633: F, t23635: F, t23657: F, t23678: F, t23685: F, t25429: F, t25540: F, t25544: F, t25717: F, t25722: F, t3120: F, t4347: F, t6687: F, t6784: F, t6797: F, t6799: F, t6800: F, t7619: F, t82661: F, t83239: F, t83240: F, t83245: F, t89019: F) -> (F, F) {
    let t89104 = t82573 * t7604;
    let t89106 = t3961 * t6746;
    let t89143 = -F::cast_from(0.48738787165873375897e-2_f64) * t89104 + F::cast_from(0.73108180748810063846e-2_f64) * t83239 * t83240 * t89106 + F::cast_from(0.10966227112321509577e-1_f64) * t83245 * t23511 * t1629 * t23678 * t14213 + t11046 * t7619 * t14630 + F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t6784 * t23685 * t4347 - F::cast_from(0.18277045187202515961e-2_f64) * t82661 - F::cast_from(0.73108180748810063846e-2_f64) * t25429 * t89019 * t25722 + F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t6799 * t14571 * t6800 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23613 * t25717 - F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t23657 * t25540 - F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t23657 * t25544 + F::cast_from(0.27415567780803773942e-2_f64) * t23633 * t23635 * t1409 * t3120 * t6800;
    (t89106, t89143)
}
