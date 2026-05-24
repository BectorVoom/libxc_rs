//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1035/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1035<F: Float>(t40699: F, t8571: F, t35691: F, t35697: F, t35699: F, t35703: F, t35705: F, t40343: F, t40350: F, t40351: F, t40354: F, t40357: F, t40459: F, t46992: F, t46995: F, t46999: F, t47004: F, t47006: F, t47008: F) -> F {
    let t47011 = t8571 * t40699;
    let t47013 = -F::cast_from(0.85129199786595678796e-5_f64) * t46992 + F::cast_from(0.10248087766267884742e-3_f64) * t35691 - F::cast_from(0.85129199786595678796e-5_f64) * t46995 + F::cast_from(0.29810146462873361018e-2_f64) * t40343 - F::cast_from(0.99317399751028291929e-5_f64) * t46999 + t40350 - F::cast_from(0.59590439850616975158e-4_f64) * t40351 + F::cast_from(0.59590439850616975158e-4_f64) * t40354 + t40357 - F::cast_from(0.19863479950205658386e-4_f64) * t47004 + F::cast_from(0.99317399751028291929e-5_f64) * t47006 - F::cast_from(0.39914139006212695213e-1_f64) * t47008 - t35697 - t35699 - t35703 - F::cast_from(0.35220688045884876043e-2_f64) * t35705 + F::cast_from(0.85129199786595678796e-5_f64) * t47011 - t40459;
    t47013
}
