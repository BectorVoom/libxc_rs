//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 256/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk256<F: Float>(t268: F, t405: F, t878: F, t154: F, t486: F, t636: F) -> (F, F, F, F) {
    let t1086 = t268 * t878 * t405;
    let t1087 = F::cast_from(0.17808333333333333333e-1_f64) * t1086;
    let t1088 = t154 * t486;
    let t1089 = F::new(1.0) / t636;
    (t1086, t1087, t1088, t1089)
}
