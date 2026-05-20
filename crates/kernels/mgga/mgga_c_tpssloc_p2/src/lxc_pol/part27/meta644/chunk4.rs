//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2202/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2202<F: Float>(t25749: F, t6698: F, t7566: F, t82573: F, t1052: F, t1065: F, t11010: F, t12648: F, t14529: F, t14545: F, t23313: F, t23329: F, t23346: F, t23369: F, t25406: F, t25429: F, t25430: F, t25731: F, t25778: F, t25811: F, t3174: F, t3207: F, t4665: F, t6687: F, t6776: F, t7600: F, t82382: F, t82432: F, t82436: F, t986: F) -> F {
    let t88182 = t6698 * t25749;
    let t88194 = F::cast_from(0.14621636149762012769e-1_f64) * t82573 * t7566;
    let t88213 = F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t986 * t88182 + F::new(4.0) * t23369 * t4665 - F::cast_from(0.18277045187202515961e-2_f64) * t82432 - F::cast_from(0.14621636149762012769e-1_f64) * t23346 * t25811 - F::cast_from(0.80418998823691070228e-1_f64) * t82382 * t7566 + t88194 + F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t23329 * t25430 * t12648 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t25406 * t23313 - t25778 * t3207 + F::new(4.0) * t14545 * t6776 + F::new(2.0) * t11010 * t7600 + F::new(4.0) * t1052 * t3174 * t25731 * t1065 + F::new(4.0) * t14529 * t6776 + t82436;
    t88213
}
