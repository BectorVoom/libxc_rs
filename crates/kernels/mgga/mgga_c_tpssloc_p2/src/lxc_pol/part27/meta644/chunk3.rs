//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2201/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2201<F: Float>(t1625: F, t23592: F, t225: F, t25791: F, t23384: F, t25413: F, t1598: F, t3014: F, t1921: F, t7577: F, t25403: F, t1066: F, t14658: F, t1599: F, t23327: F, t23332: F, t23365: F, t23594: F, t23722: F, t25424: F, t25784: F, t25797: F, t25826: F, t3010: F, t4660: F, t6687: F, t6704: F, t6705: F, t7553: F, t82400: F, t82417: F, t82426: F, t83424: F, t83453: F) -> (F, F) {
    let t88138 = t23592 * t1625;
    let t88145 = t25791 * t225;
    let t88152 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25413;
    let t88155 = t1598 * t3014;
    let t88162 = t7577 * t1921;
    let t88167 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25403;
    let t88179 = F::cast_from(0.36554090374405031923e-2_f64) * t6687 * t88138 * t23594 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t23365 * t25826 - F::new(2.0) * t88145 * t1066 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t83424 * t7553 - t88152 + F::cast_from(0.54831135561607547884e-2_f64) * t82400 - t4660 * t23722 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t88155 * t25797 + F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t1599 * t83453 + F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88162 * t23332 - t88167 - F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t82417 * t25424 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t6704 * t6705 * t14658 + F::cast_from(0.91385225936012579807e-3_f64) * t82426 + F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t3010 * t25784;
    (t88155, t88179)
}
