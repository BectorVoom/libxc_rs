//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2240/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2240<F: Float>(t7604: F, t82632: F, t25723: F, t88810: F, t1409: F, t3040: F, t1539: F, t6746: F, t82655: F, t14220: F, t7581: F, t11034: F, t1599: F, t1629: F, t23346: F, t23518: F, t23604: F, t23620: F, t23633: F, t25467: F, t25567: F, t25659: F, t25708: F, t3186: F, t4673: F, t6687: F, t82382: F, t82653: F, t82789: F, t83233: F, t83245: F, t83265: F, t89106: F) -> (F, F) {
    let t89366 = t82632 * t7604;
    let t89369 = F::cast_from(0.24369393582936687948e-2_f64) * t88810 * t25723;
    let t89375 = t1409 * t3040;
    let t89395 = t82655 * t1539 * t6746;
    let t89399 = t82655 * t7581 * t14220;
    let t89402 = F::cast_from(0.26806332941230356743e-1_f64) * t82382 * t7604 - F::cast_from(0.60923483957341719871e-3_f64) * t89366 + t89369 + F::new(4.0) * t11034 * t25708 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t1599 * t23620 - F::cast_from(0.27415567780803773942e-2_f64) * t83245 * t83265 * t89375 * t23604 + F::new(4.0) * t3186 * t25567 * t4673 + F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25467 - F::cast_from(0.27415567780803773942e-2_f64) * t82789 - F::cast_from(0.54831135561607547884e-2_f64) * t83245 * t23518 * t1629 * t25659 * t14220 - F::cast_from(0.10966227112321509577e-1_f64) * t23633 * t83233 * t89106 - F::cast_from(0.54831135561607547884e-2_f64) * t82653 * t89395 - F::cast_from(0.54831135561607547884e-2_f64) * t82653 * t89399;
    (t89375, t89402)
}
