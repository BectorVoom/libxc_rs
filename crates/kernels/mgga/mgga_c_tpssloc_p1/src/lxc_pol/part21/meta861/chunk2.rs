//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3124/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3124<F: Float>(t15419: F, t18215: F, t3447: F, t18469: F, t44525: F, t18206: F, t52133: F, t15324: F, t15327: F, t15376: F, t15379: F, t15391: F, t44529: F, t44558: F, t4900: F, t63386: F, t63394: F) -> F {
    let t64624 = t3447 * t15419 * t18215;
    let t64627 = t3447 * t44525 * t18469;
    let t64632 = t3447 * t52133 * t18206;
    let t64634 = -F::cast_from(0.2962962962962962963e-2_f64) * t15376 * t15327 + F::cast_from(0.22222222222222222221e-2_f64) * t3447 * t4900 * t63386 - F::cast_from(0.14814814814814814815e-2_f64) * t15376 * t15324 + F::cast_from(0.74074074074074074072e-3_f64) * t3447 * t4900 * t63394 - F::cast_from(0.37037037037037037036e-3_f64) * t3447 * t44529 * t18469 - F::cast_from(0.14814814814814814814e-2_f64) * t15376 * t15379 - F::cast_from(0.37037037037037037036e-3_f64) * t3447 * t44558 * t18469 + F::cast_from(0.49382716049382716048e-3_f64) * t64624 - F::cast_from(0.24691358024691358024e-3_f64) * t64627 + F::cast_from(0.1975308641975308642e-2_f64) * t15376 * t15391 - F::cast_from(0.5761316872427983539e-3_f64) * t64632;
    t64634
}
