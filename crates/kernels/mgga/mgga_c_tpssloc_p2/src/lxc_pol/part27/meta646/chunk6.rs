//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2225/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2225<F: Float>(t1054: F, t4693: F, t13783: F, t1926: F, t221: F, t25432: F, t10164: F, t10170: F, t1052: F, t1065: F, t14658: F, t1955: F, t23327: F, t23329: F, t23330: F, t23369: F, t23402: F, t23581: F, t25429: F, t25705: F, t25749: F, t25757: F, t25801: F, t25810: F, t2771: F, t2780: F, t3174: F, t388: F, t3966: F, t4664: F, t4694: F, t6687: F, t6815: F, t7554: F, t7600: F, t82382: F, t83285: F, t83287: F, t884: F, t990: F) -> (F, F, F) {
    let t88804 = t1054 * t4693;
    let t88810 = t1926 * t221 * t13783;
    let t88812 = F::cast_from(0.24369393582936687948e-2_f64) * t88810 * t25432;
    let t88827 = F::cast_from(0.14621636149762012769e-1_f64) * t83285 + F::cast_from(0.14621636149762012769e-1_f64) * t83287 + F::new(2.0) * t10170 * t7600 + F::new(2.0) * t1052 * t3174 * t1955 * t14658 + F::new(2.0) * t990 * t25705 * t388 - F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t25810 * t23402 - F::new(2.0) * t23369 * t4694 - F::cast_from(0.27415567780803773942e-2_f64) * t23327 * t23329 * t25749 * t2780 - F::cast_from(0.36554090374405031923e-2_f64) * t25429 * t23329 * t25749 * t2771 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23329 * t88804 * t884 + t88812 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23329 * t23330 * t3966 * t1065 - F::new(12.0) * t25757 * t10164 * t6815 * t4664 + F::cast_from(0.26806332941230356743e-1_f64) * t82382 * t7554 + F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t23581 * t25801;
    (t88804, t88810, t88827)
}
