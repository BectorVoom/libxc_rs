//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1456/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1456<F: Float>(t44249: F, t50846: F, t71470: F, t71472: F, t71474: F, t78026: F, t78029: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F, t78078: F, t78080: F) -> F {
    let t78839 = -F::cast_from(0.12349037037037037037e1_f64) * t50846 - F::cast_from(0.12349037037037037037e0_f64) * t71470 + F::cast_from(0.55570666666666666668e0_f64) * t71472 - F::new(0.166712e1) * t71474 + t44249 - F::new(0.52945875e1) * t78026 + F::cast_from(0.2366859375e0_f64) * t78029 - F::cast_from(0.13772666666666666667e1_f64) * t78033 + F::cast_from(0.34431666666666666667e1_f64) * t78037 - F::new(0.123954e2) * t78041 + F::new(0.185931e2) * t78045 + F::new(0.41318e1) * t78049 + F::new(0.6311625e0) * t78078 - F::cast_from(0.6618234375e1_f64) * t78080;
    t78839
}
