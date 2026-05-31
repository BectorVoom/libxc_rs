//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 897/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk897<F: Float>(t112: F, t32262: F, t2039: F, t23938: F, t26977: F, t31237: F, t31239: F, t32206: F, t32235: F, t671: F, t7042: F, t7056: F, t8446: F, t9012: F) -> (F, F) {
    let t32263 = t32262 * t112;
    let t32278 = F::cast_from(4.0_f64) * t2039 * t23938 + F::cast_from(4.0_f64) * t2039 * t26977 + F::cast_from(2.0_f64) * t32235 * t671 + F::cast_from(4.0_f64) * t7042 * t7056 + F::cast_from(4.0_f64) * t7056 * t9012 + t31237 + t31239 + F::cast_from(2.0_f64) * t32206 + t32263 + t8446;
    (t32263, t32278)
}
