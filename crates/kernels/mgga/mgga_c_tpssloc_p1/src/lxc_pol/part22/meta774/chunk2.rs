//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2649/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2649<F: Float>(t54312: F, t39328: F, t39339: F, t39341: F, t1388: F, t6347: F, t54325: F, t20416: F, t3918: F, t3919: F, t39338: F, t39346: F, t39349: F, t39356: F, t39360: F, t5161: F) -> (F, F, F, F, F, F) {
    let t74024 = F::cast_from(72.0_f64) * t54312;
    let t74026 = F::cast_from(0.16265371950452609763e-1_f64) * t39328;
    let t74027 = F::cast_from(0.35089341735807877242e1_f64) * t39339;
    let t74028 = F::cast_from(0.51947577317044391277e2_f64) * t39341;
    let t74032 = t6347 * t1388;
    let t74036 = F::cast_from(0.17090684152272775384e-2_f64) * t54325;
    let t74037 = F::cast_from(3.0_f64) * t20416 * t3918 * t3919 - F::cast_from(9.0_f64) * t3918 * t5161 * t74032 - t39338 + t39346 + t39349 + t39356 + t39360 + t74026 + t74027 - t74028 - t74036;
    (t74024, t74026, t74027, t74028, t74036, t74037)
}
