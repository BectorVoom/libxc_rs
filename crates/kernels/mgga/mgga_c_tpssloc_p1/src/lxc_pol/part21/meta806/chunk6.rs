//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2804/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2804<F: Float>(t5624: F, t9993: F, t5628: F, t16985: F, t2697: F, t1516: F, t47275: F, t47278: F, t9601: F, t2700: F, t57043: F, t247: F, t4181: F) -> (F, F, F, F, F, F, F, F) {
    let t59251 = t9993 * t5624;
    let t59255 = t9993 * t5628;
    let t59257 = t2697 * t16985;
    let t59259 = t47275 * t1516;
    let t59261 = t47278 * t1516;
    let t59263 = t9601 * t5628;
    let t59265 = t57043 * t2700;
    let t59267 = t247 * t4181;
    (t59251, t59255, t59257, t59259, t59261, t59263, t59265, t59267)
}
