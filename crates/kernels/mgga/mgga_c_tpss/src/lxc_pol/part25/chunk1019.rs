//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1019/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1019<F: Float>(t57: F, t2232: F, t4579: F, t13335: F, t14096: F, t3431: F, t3582: F, t581: F, t81: F, t14095: F, t162: F, t187: F, t8101: F, zeta_threshold: F) -> (F, F, F) {
    let t155 = t57 <= zeta_threshold;
    let t14101 = t2232 * t4579;
    let t14107 = piecewise3::<F>(t155, F::new(0.0), F::new(8.0) / F::new(27.0) * t14096 * t581 + F::new(8.0) / F::new(9.0) * t3582 * t3431 + F::new(4.0) / F::new(9.0) * t14101 * t581 - F::new(4.0) / F::new(3.0) * t81 * t13335);
    let t14108 = t14095 + t14107;
    let t14109 = t14108 * t162;
    let t14111 = F::cast_from(0.19751673498613801407e-1_f64) * t14109 * t187;
    let t14112 = F::new(4.0) * t8101;
    (t14108, t14111, t14112)
}
