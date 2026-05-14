//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 973/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk973<F: Float>(t45: F, t1163: F, t3537: F, t1338: F, t3166: F, t8006: F, t8024: F, t8035: F, t1289: F, t8050: F, t2225: F, t3431: F, t10353: F, t1985: F, t1992: F, t3575: F, t581: F, t78: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t10461 = t1163 * t3537;
    let t10464 = t3166 * t1338;
    let t10470 = 0.11696447245269292414e1 * t8006;
    let t10471 = 2.0 * t8024;
    let t10472 = 0.5848223622634646207e0 * t8035;
    let t10473 = t8050 * t1289;
    let t10476 = t2225 * t3431;
    let t10484 = piecewise3(t151, 0.0, -8.0 / 27.0 * t10473 * t1985 + 8.0 / 9.0 * t10476 * t581 + 4.0 / 9.0 * t3575 * t1992 + 4.0 / 3.0 * t78 * t10353);
    (t10461, t10464, t10470, t10471, t10472, t10484)
}
