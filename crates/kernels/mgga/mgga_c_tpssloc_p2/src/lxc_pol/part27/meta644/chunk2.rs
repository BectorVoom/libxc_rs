//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2200/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2200<F: Float>(t23384: F, t25785: F, t25447: F, t1625: F, t6733: F, t23328: F, t6705: F, t13742: F, t1956: F, t23327: F, t23331: F, t23346: F, t23372: F, t23728: F, t25424: F, t25429: F, t25431: F, t25757: F, t25758: F, t25810: F, t4337: F, t4342: F, t4665: F, t50622: F, t6687: F, t6691: F, t82380: F, t82502: F) -> F {
    let t88100 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25785;
    let t88102 = F::cast_from(0.54831135561607547884e-2_f64) * t23384 * t25447;
    let t88105 = t6733 * t1625;
    let t88112 = t23328 * t6705;
    let t88137 = t88100 + t88102 - F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25785 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t88105 * t6691 - F::cast_from(12.0_f64) * t25757 * t25758 * t13742 + F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t88112 * t4342 * t23331 - F::cast_from(0.73108180748810063846e-2_f64) * t25429 * t88112 * t4337 * t23331 + F::cast_from(0.10966227112321509577e-1_f64) * t23327 * t82502 * t25424 - F::cast_from(0.73108180748810063846e-2_f64) * t25429 * t82502 * t25431 - F::cast_from(0.54831135561607547884e-2_f64) * t82380 + F::cast_from(4.0_f64) * t23372 * t4665 - F::cast_from(2.0_f64) * t50622 * t1956 - F::cast_from(0.43864908449286038306e-1_f64) * t23346 * t25447 + F::cast_from(0.27415567780803773942e-2_f64) * t6687 * t25810 * t23728;
    t88137
}
