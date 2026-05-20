//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 850/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk850<F: Float>(t4433: F, t932: F, t1568: F, t2888: F, t931: F, t2766: F, t2892: F, t4335: F, t4340: F, t4345: F, t4349: F, t324: F) -> (F, F, F, F, F) {
    let t4434 = t4433 * t932;
    let t4437 = t1568 * t2888;
    let t4438 = t4437 * t931;
    let t4446 = t2892 + F::cast_from(0.30902777777777777778e-2_f64) * t2766 + F::cast_from(0.30902777777777777778e-2_f64) * t4335 - F::cast_from(0.61805555555555555555e-2_f64) * t4340 + F::cast_from(0.18541666666666666667e-1_f64) * t4345 - F::cast_from(0.92708333333333333333e-2_f64) * t4349;
    let t4447 = t4446 * t324;
    (t4434, t4437, t4438, t4446, t4447)
}
