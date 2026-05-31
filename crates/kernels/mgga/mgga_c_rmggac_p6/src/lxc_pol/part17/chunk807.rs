//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 807/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk807<F: Float>(t1965: F, t9085: F, t1969: F, t2305: F, t35654: F, t16502: F, t8516: F, t5016: F, t9000: F, t1605: F, t1986: F, t8817: F, t942: F) -> (F, F, F, F, F, F) {
    let t39392 = t9085 * t1965;
    let t39393 = t39392 * t1969;
    let t39405 = t35654 * t2305;
    let t39406 = F::cast_from(0.19863479950205658386e-4_f64) * t39405;
    let t39437 = t8516 * t16502;
    let t39451 = t5016 * t9000;
    let t39452 = F::cast_from(0.15965655602485078085e0_f64) * t39451;
    let t39490 = t1986 * t1605;
    let t39506 = F::cast_from(0.4726e1_f64) * t942 * t8817;
    (t39393, t39406, t39437, t39452, t39490, t39506)
}
