//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 584/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk584<F: Float>(t7780: F, t1347: F, t703: F, t2244: F, t275: F, t7908: F, t7910: F, t7818: F, t7820: F, t2227: F, t874: F, t7937: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8197 = F::cast_from(0.15965655602485078085e0_f64) * t7780;
    let t8201 = t1347 * t703;
    let t8208 = t275 * t2244;
    let t8209 = F::cast_from(2.0_f64) * t8208;
    let t8221 = F::cast_from(0.39726959900411316772e-4_f64) * t7908;
    let t8222 = F::cast_from(0.11918087970123395032e-3_f64) * t7910;
    let t8242 = F::cast_from(0.2927036860455597649e0_f64) * t7818;
    let t8243 = F::cast_from(0.66671395154821946452e-1_f64) * t7820;
    let t8264 = t874 * t2227;
    let t8303 = F::cast_from(0.1440846329149835838e-2_f64) * t7937;
    (t8197, t8201, t8209, t8221, t8222, t8242, t8243, t8264, t8303)
}
