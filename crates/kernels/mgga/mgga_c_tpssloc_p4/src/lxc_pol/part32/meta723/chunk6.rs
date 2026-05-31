//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2312/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2312<F: Float>(t1186: F, t11881: F, t1751: F, t19145: F, t19165: F, t24812: F, t24814: F, t24815: F, t27517: F, t27533: F, t27549: F, t27550: F, t29708: F, t29711: F, t29719: F, t29726: F, t3242: F, t3610: F, t3624: F, t3961: F, t5068: F, t5079: F, t6146: F, t7283: F, t7381: F, t94395: F, t95092: F, t95163: F, t95165: F, t95192: F, t95213: F) -> F {
    let t103918 = -F::cast_from(2.0_f64) * t3624 * t29719 * t5079 + F::cast_from(2.0_f64) * t3610 * t29711 * t5068 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t6146 * t7381 + F::cast_from(0.14621636149762012769e-1_f64) * t95092 * t27533 - t95163 - F::cast_from(0.14621636149762012769e-1_f64) * t94395 * t27517 + F::cast_from(0.16449340668482264365e-1_f64) * t24812 * t24814 * t19145 * t24815 + F::cast_from(0.73108180748810063845e-2_f64) * t27549 * t27550 * t1751 * t3242 * t3961 + t95165 - t95192 + t95213 + F::cast_from(6.0_f64) * t11881 * t29708 * t19165 - F::cast_from(0.82246703342411321825e-2_f64) * t7283 * t1186 * t29726;
    t103918
}
