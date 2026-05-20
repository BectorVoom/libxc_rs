//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2259/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2259<F: Float>(t13065: F, t1492: F, t1527: F, t1912: F, t23281: F, t25160: F, t25188: F, t25329: F, t259: F, t2597: F, t2713: F, t2718: F, t28406: F, t28432: F, t4301: F, t5658: F, t58143: F, t59466: F, t59519: F, t7538: F, t798: F, t82147: F, t82154: F, t855: F, t858: F, t87029: F, t87050: F, t87754: F, t98315: F, t98319: F, t98322: F, t98370: F, t98409: F, t98450: F, t98497: F, t98536: F, t98566: F, t98587: F, t98886: F) -> F {
    let t98913 = -t2597 * t28432 - F::cast_from(0.16449340668482264365e-1_f64) * t98315 - F::cast_from(0.16449340668482264365e-1_f64) * t98319 + F::cast_from(0.82246703342411321825e-2_f64) * t98322 - t855 * t858 * (t98370 + t98409 + t98450 + t98497 + t98536 + t98566 + t98587 + t98886) - F::new(2.0) * t13065 * t7538 - F::new(2.0) * t59519 * t1912 + t87029 - F::cast_from(0.26044789391763585244e-1_f64) * t82147 + F::new(4.0) * t855 * t2718 * t25329 * t1527 - t23281 * t5658 - t59466 * t1912 - F::cast_from(0.23029076935875170111e0_f64) * t87050 - t82154 - t87754 - t2713 * t28432 + F::new(2.0) * t1492 * t25160 * t259 + t798 * t28406 * t259 - F::new(2.0) * t25188 * t4301 - t58143 * t1912;
    t98913
}
