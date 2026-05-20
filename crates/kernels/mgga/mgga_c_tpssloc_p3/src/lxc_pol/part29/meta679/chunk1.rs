//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2277/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2277<F: Float>(t24600: F, t24615: F, t1090: F, t12648: F, t14165: F, t2128: F, t24589: F, t24590: F, t24601: F, t24603: F, t27411: F, t27433: F, t27549: F, t27774: F, t4728: F, t5059: F, t7287: F, t85661: F, t85669: F, t86403: F, t94349: F, t94354: F, t94358: F, t94363: F, t94365: F, t94369: F, t94374: F) -> F {
    let t94378 = t24600 * t24615;
    let t94385 = -F::cast_from(0.3289868133696452873e-1_f64) * t2128 * t24590 * t27411 + F::cast_from(0.36554090374405031923e-2_f64) * t27549 * t24601 * t27774 * t12648 + F::cast_from(0.21932454224643019154e-1_f64) * t27549 * t24601 * t94349 * t14165 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94354 * t7287 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94358 * t7287 + t94363 + t94365 - F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t86403 * t27433 + F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t94369 * t4728 * t24603 + F::cast_from(0.54831135561607547884e-2_f64) * t24589 * t94374 * t7287 - F::cast_from(0.10966227112321509577e-1_f64) * t24589 * t94378 * t5059 * t1090 + F::cast_from(0.36554090374405031922e-2_f64) * t85661 - F::cast_from(0.27415567780803773942e-2_f64) * t85669;
    t94385
}
