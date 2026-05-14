//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 746/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk746<F: Float>(t30: F, t33: F, t1197: F, t3217: F, t4578: F, t5328: F, t1201: F, t3225: F, t5059: F, t5335: F, zeta_threshold: F) -> (F,) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t5358 = piecewise3(t31, 0.0, -2.0 / 9.0 * t3217 * t5328 + 2.0 / 3.0 * t1197 * t4578);
    let t5364 = piecewise3(t34, 0.0, -2.0 / 9.0 * t3225 * t5335 + 2.0 / 3.0 * t1201 * t5059);
    let t5366 = t5358 / 2.0 + t5364 / 2.0;
    (t5366,)
}
