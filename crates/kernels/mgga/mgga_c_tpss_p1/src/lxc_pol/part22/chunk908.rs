//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 908/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk908<F: Float>(t159: F, t8185: F, t216: F, t570: F, t66: F, t235: F, t238: F, t242: F, t232: F, t2169: F, t2367: F, t2215: F, t2218: F) -> (F, F, F, F, F, F, F) {
    let t8186 = t8185 * t159;
    let t8188 = F::cast_from(455.0_f64) / F::cast_from(1296.0_f64) * t8186 * t216;
    let t8199 = F::cast_from(1.0_f64) / t66 / t570;
    let t8200 = t8199 * t235;
    let t8202 = t8200 * t238 * t242;
    let t8204 = F::cast_from(595.0_f64) / F::cast_from(10368.0_f64) * t232 * t8202;
    let t8205 = t2169 * t2367;
    let t8212 = t2218 * t2215;
    (t8186, t8188, t8200, t8202, t8204, t8205, t8212)
}
