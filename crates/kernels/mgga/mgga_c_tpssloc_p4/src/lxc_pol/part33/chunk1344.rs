//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1344/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1344<F: Float>(t1052: F, t1599: F, t1634: F, t18074: F, t1955: F, t21662: F, t21676: F, t21691: F, t23327: F, t23329: F, t23330: F, t23394: F, t25429: F, t25442: F, t25755: F, t25810: F, t28485: F, t28491: F, t28499: F, t28679: F, t3174: F, t4660: F, t5398: F, t5944: F, t6687: F, t6704: F, t7625: F, t82481: F, t99301: F, t99330: F, t99400: F) -> F {
    let t105971 = -F::cast_from(0.54831135561607547883e-2_f64) * t99301 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t25810 * t28499 + F::cast_from(0.49348022005446793095e-1_f64) * t6687 * t1599 * t99400 + F::cast_from(12.0_f64) * t4660 * t28485 + F::cast_from(0.49348022005446793095e-1_f64) * t6687 * t6704 * t23394 * t21691 + F::cast_from(2.0_f64) * t1052 * t3174 * t1955 * t21662 - F::cast_from(3.0_f64) * t4660 * t28679 + F::cast_from(0.54831135561607547883e-2_f64) * t99330 - F::cast_from(0.10966227112321509577e-1_f64) * t25429 * t25442 * t28491 - F::cast_from(0.82246703342411321826e-2_f64) * t23327 * t23329 * t23330 * t5398 * t1634 - F::cast_from(3.0_f64) * t25755 * t5944 - F::cast_from(0.49348022005446793095e-1_f64) * t6687 * t6704 * t82481 * t21676 - F::cast_from(3.0_f64) * t18074 * t7625;
    t105971
}
