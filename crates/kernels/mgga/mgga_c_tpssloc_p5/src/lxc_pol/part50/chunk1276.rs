//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1276/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1276<F: Float>(t19577: F, t22574: F, t36533: F, t8449: F, t8944: F, t26164: F, t120103: F, t120104: F, t120107: F, t120108: F, t120111: F, t120114: F, t120138: F, t120166: F, t120171: F, t120173: F, t120176: F, t120177: F, t120658: F, t120659: F, t1774: F, t1976: F, t26098: F, t31029: F, t5361: F, t574: F, t6862: F, t7451: F, t8447: F) -> F {
    let t120663 = F::cast_from(6.0_f64) * t22574 * t36533 * t19577;
    let t120664 = t8449 * t8944;
    let t120665 = t120664 * t26164;
    let t120667 = t8447 * t5361 + t120103 - F::cast_from(6.0_f64) * t120104 + t120107 - F::cast_from(4.0_f64) * t120108 - t120111 - t120114 - t31029 * t1774 - F::cast_from(2.0_f64) * t26098 * t1976 - F::cast_from(2.0_f64) * t7451 * t6862 + (t120138 + t120166) * t574 + t120171 + F::cast_from(12.0_f64) * t120173 - t120176 + F::cast_from(2.0_f64) * t120177 + t120658 - F::cast_from(2.0_f64) * t120659 + t120663 + F::cast_from(4.0_f64) * t120665;
    t120667
}
