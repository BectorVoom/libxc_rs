//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2421/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2421<F: Float>(t17366: F, t4488: F, t959: F, t21091: F, t2940: F, t21373: F, t17930: F, t4483: F, t17564: F, t48890: F, t1068: F, t21376: F, t43637: F, t4700: F, t69003: F, t69005: F, t69011: F, t69014: F, t69018: F) -> (F, F, F, F, F, F) {
    let t69021 = F::cast_from(0.35089341735807877242e1_f64) * t959 * t4488 * t17366;
    let t69023 = F::cast_from(0.35089341735807877242e1_f64) * t2940 * t21091;
    let t69025 = F::cast_from(0.35089341735807877242e1_f64) * t2940 * t21373;
    let t69027 = F::cast_from(0.10389515463408878255e3_f64) * t4483 * t17930;
    let t69030 = F::cast_from(0.30762056574649219974e4_f64) * t959 * t17564 * t48890;
    let t69031 = -F::new(6.0) * t1068 * t21376 * t43637 * t4700 - t69003 + t69005 - t69011 - t69014 + t69018 + t69021 - t69023 + t69025 - t69027 - t69030;
    (t69021, t69023, t69025, t69027, t69030, t69031)
}
