//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 594/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk594<F: Float>(t30: F, t33: F, t3426: F, t70: F, t1989: F, zeta_threshold: F) -> (F, F) {
    let t31 = t30 <= zeta_threshold;
    let t34 = t33 <= zeta_threshold;
    let t3427 = t3426 * t70;
    let t3431 = piecewise5::<F>(t31, F::new(0.0), t34, F::new(0.0), F::new(2.0) * t1989);
    (t3427, t3431)
}
