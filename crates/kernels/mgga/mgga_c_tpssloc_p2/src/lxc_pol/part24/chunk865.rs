//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 865/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk865<F: Float>(t118: F, t181: F, t2454: F, t2459: F, t2460: F, t2462: F, t2471: F, t2472: F, t2477: F, t2479: F, t2480: F, t2490: F, t2494: F, t2495: F, t2505: F, t2510: F, t2513: F, t268: F, t676: F, t730: F, t732: F, t747: F, t9697: F, t9799: F, t9803: F, t9810: F, t9814: F, t9820: F, t9824: F, t9828: F, t9844: F, t9847: F, t9853: F, t9859: F) -> F {
    let t9860 = F::cast_from(0.32530743900905219526e-1_f64) * t268 * t9799 * t2495 + F::cast_from(0.21687162600603479684e-1_f64) * t268 * t9803 * t747 - F::cast_from(0.16265371950452609763e-1_f64) * t268 * t2490 * t2505 - F::cast_from(0.48159733137676571078e0_f64) * t268 * t9810 * t2513 + F::cast_from(0.68493333333333333332e-1_f64) * t268 * t9814 * t732 + t9820 + t9824 - F::cast_from(0.51369999999999999999e-1_f64) * t268 * t2454 * t2472 - F::cast_from(0.16522625736956710527e1_f64) * t268 * t9828 * t2480 + F::cast_from(0.10274e0_f64) * t268 * t676 * t2459 * t2462 + F::cast_from(0.96491876992155210402e2_f64) * t2477 * t2471 * t2479 * t730 - F::cast_from(6.0_f64) * t2460 * t732 * t2471 + F::cast_from(0.51947577317044391277e2_f64) * t2510 * t9844 - F::cast_from(0.35089341735807877242e1_f64) * t2494 * t9847 - t9853 + F::cast_from(0.56968947174242584612e-3_f64) * t118 * t9697 * t181 - t9859;
    t9860
}
