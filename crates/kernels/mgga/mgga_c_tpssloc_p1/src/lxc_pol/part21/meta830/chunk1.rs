//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2927/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2927<F: Float>(t17938: F, t2940: F, t13663: F, t4483: F, t14259: F, t41825: F, t5774: F, t959: F, t17566: F, t3213: F, t43637: F, t4700: F, t5950: F, t60359: F, t60371: F, t60374: F, t60377: F, t60381: F, t60384: F, t60387: F, t60391: F, t60394: F) -> (F, F, F, F, F) {
    let t60930 = F::cast_from(0.23392894490538584828e1_f64) * t2940 * t17938;
    let t60932 = F::cast_from(0.46785788981077169656e1_f64) * t4483 * t13663;
    let t60936 = F::cast_from(0.12304822629859687989e5_f64) * t959 * t41825 * t5774 * t14259;
    let t60938 = F::cast_from(0.20508037716432813316e4_f64) * t2940 * t17566;
    let t60939 = -F::new(6.0) * t3213 * t43637 * t4700 * t5950 - t60359 - t60371 - t60374 + t60377 + t60381 + t60384 + t60387 + t60391 + t60394 + t60930 + t60932 + t60936 - t60938;
    (t60930, t60932, t60936, t60938, t60939)
}
