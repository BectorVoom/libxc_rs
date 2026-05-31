//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1360/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1360<F: Float>(t121007: F, t1874: F, t27188: F, t6535: F, t31304: F, t7688: F, t31537: F, t7796: F, t31540: F, t27163: F, t8526: F, t119832: F, t26161: F, t26558: F) -> (F, F, F, F, F, F, F) {
    let t121009 = F::cast_from(2.0_f64) * t121007 * t1874;
    let t121019 = F::cast_from(2.0_f64) * t27188 * t6535;
    let t121132 = F::cast_from(3.0_f64) * t31304 * t7688;
    let t121134 = F::cast_from(2.0_f64) * t31537 * t7796;
    let t121136 = F::cast_from(2.0_f64) * t31540 * t7796;
    let t121138 = F::cast_from(2.0_f64) * t8526 * t27163;
    let t121142 = F::cast_from(2.0_f64) * t26161 * t26558 * t119832;
    (t121009, t121019, t121132, t121134, t121136, t121138, t121142)
}
