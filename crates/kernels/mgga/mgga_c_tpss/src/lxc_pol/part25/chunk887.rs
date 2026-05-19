//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 887/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk887<F: Float>(t235: F, t8199: F, t238: F, t242: F, t232: F, t2215: F, t2218: F, t2345: F, t2206: F, t651: F, t2348: F, t123: F, t727: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t8200 = t8199 * t235;
    let t8202 = t8200 * t238 * t242;
    let t8204 = F::new(595.0) / F::new(10368.0) * t232 * t8202;
    let t8212 = t2218 * t2215;
    let t8218 = t2218 * t2345;
    let t8220 = t651 * t2206;
    let t8222 = F::cast_from(0.16265371950452609763e-1_f64) * t2348 * t8220;
    let t8223 = t651 * t2215;
    let t8225 = F::cast_from(0.48159733137676571078e0_f64) * t2348 * t8223;
    let t8226 = t727 * t123;
    (t8200, t8202, t8204, t8212, t8218, t8220, t8222, t8223, t8225, t8226)
}
