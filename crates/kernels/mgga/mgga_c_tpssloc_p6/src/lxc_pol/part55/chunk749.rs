//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 749/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk749<F: Float>(t577: F, t671: F, t7014: F, t7017: F, t7019: F, t7415: F, t7423: F, t33: F, t3953: F, t1437: F, t79: F, t72: F) -> (F, F, F, F) {
    let t7426 = F::cast_from(0.45e1_f64) * t7415 * t577 + F::cast_from(0.135e2_f64) * t7423 * t671 + t7014 + t7017 + t7019;
    let t7428 = t3953 * t33;
    let t7431 = t79 * t1437;
    let t7432 = t72 * t7431;
    (t7426, t7428, t7431, t7432)
}
