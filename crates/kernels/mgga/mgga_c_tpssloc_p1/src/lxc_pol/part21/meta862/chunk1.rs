//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3130/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3130<F: Float>(t11588: F, t6144: F, t3447: F, t3451: F, t15402: F, t18237: F, t15376: F, t15395: F, t15406: F, t3449: F, t4900: F, t4908: F, t63294: F, t63311: F, t63372: F, t63378: F, t64756: F, t64765: F, t64770: F, t64773: F, t64775: F) -> F {
    let t64779 = t11588 * t6144;
    let t64781 = t3447 * t64779 * t3451;
    let t64784 = t3447 * t15402 * t18237;
    let t64786 = -F::cast_from(0.86419753086419753084e-3_f64) * t3447 * t15395 * t63372 - F::cast_from(0.1037037037037037037e-1_f64) * t3447 * t15395 * t63378 + F::cast_from(0.74074074074074074072e-3_f64) * t3447 * t4900 * t63311 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t3449 * t64756 - F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t4908 * t63294 + F::cast_from(0.18518518518518518518e-3_f64) * t64765 - F::cast_from(0.39506172839506172838e-2_f64) * t15376 * t15406 + F::cast_from(0.37037037037037037036e-3_f64) * t64770 + F::cast_from(0.37037037037037037036e-3_f64) * t64773 + F::cast_from(0.55555555555555555554e-3_f64) * t3447 * t64775 * t3451 + F::cast_from(0.18518518518518518518e-3_f64) * t64781 - F::cast_from(0.37037037037037037036e-3_f64) * t64784;
    t64786
}
