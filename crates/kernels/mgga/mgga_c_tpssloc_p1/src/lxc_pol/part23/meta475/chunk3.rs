//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1423/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1423<F: Float>(t44027: F, t50846: F, t71470: F, t71472: F, t71474: F, t78026: F, t78029: F, t78033: F, t78037: F, t78041: F, t78045: F, t78049: F, t78078: F, t78080: F) -> F {
    let t78177 = -F::cast_from(0.97370864197530864199e0_f64) * t50846 - F::cast_from(0.97370864197530864196e-1_f64) * t71470 + F::cast_from(0.43816888888888888888e0_f64) * t71472 - F::cast_from(0.13145066666666666666e1_f64) * t71474 + t44027 - F::new(0.28483875e1) * t78026 + F::cast_from(0.1151859375e0_f64) * t78029 - F::cast_from(0.79724444444444444444e0_f64) * t78033 + F::cast_from(0.19931111111111111111e1_f64) * t78037 - F::cast_from(0.71752000000000000001e1_f64) * t78041 + F::new(0.107628e2) * t78045 + F::cast_from(0.23917333333333333333e1_f64) * t78049 + F::new(0.3071625e0) * t78078 - F::cast_from(0.3560484375e1_f64) * t78080;
    t78177
}
