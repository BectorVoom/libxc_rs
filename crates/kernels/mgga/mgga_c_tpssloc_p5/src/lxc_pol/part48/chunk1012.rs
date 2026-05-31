//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 1012/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk1012<F: Float>(t23929: F, t8526: F, t1307: F, t22574: F, t26558: F, t31775: F, t22607: F, t8641: F, t1983: F, t31669: F, t6999: F, t7015: F, t84033: F) -> (F, F, F, F, F) {
    let t115948 = F::cast_from(4.0_f64) * t8526 * t23929;
    let t115959 = F::cast_from(12.0_f64) * t22574 * t26558 * t31775 * t1307;
    let t115965 = t22607 * t8641;
    let t115968 = F::cast_from(2.0_f64) * t1983 * t31669 * t6999;
    let t115978 = F::cast_from(54.0_f64) * t84033 * t7015;
    (t115948, t115959, t115965, t115968, t115978)
}
