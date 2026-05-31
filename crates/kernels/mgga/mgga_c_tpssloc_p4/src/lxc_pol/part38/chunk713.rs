//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 713/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk713<F: Float>(t3584: F, t61: F, t248: F, t3243: F, t1174: F, t1213: F, t1218: F, t1227: F, t1232: F, t3490: F, t3496: F, t3506: F, t3511: F, t3515: F, t3518: F, t3524: F, t3527: F, t3531: F, t3536: F, t3542: F, t3543: F, t3547: F, t3549: F, t3552: F, t3557: F, t3562: F, t3567: F, t3573: F, t3577: F, t3580: F, t488: F) -> (F, F, F) {
    let t3585 = t61 * t3584;
    let t3587 = t248 * t3585 * t3243;
    let t3590 = -t3490 * t1232 / F::cast_from(2304.0_f64) + t1213 * t3496 / F::cast_from(3072.0_f64) + t3506 * t3511 / F::cast_from(1536.0_f64) - t3515 * t3518 / F::cast_from(3072.0_f64) - t3524 / F::cast_from(3456.0_f64) - t1227 * t3527 / F::cast_from(4608.0_f64) - t1227 * t3531 / F::cast_from(2304.0_f64) + t3536 * t1218 / F::cast_from(1536.0_f64) - t3542 + t3543 / F::cast_from(2304.0_f64) - t3547 - t3549 / F::cast_from(432.0_f64) - t1174 * t3552 / F::cast_from(288.0_f64) - t1174 * t3557 / F::cast_from(144.0_f64) + t1174 * t3562 / F::cast_from(216.0_f64) + t3567 * t488 / F::cast_from(3072.0_f64) + t3573 / F::cast_from(2304.0_f64) - t3577 * t3580 / F::cast_from(2304.0_f64) + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1227 * t3587;
    (t3585, t3587, t3590)
}
