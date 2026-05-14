//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 778/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk778<F: Float>(t40928: F, t649: F, t8976: F, t35960: F, t8947: F, t11704: F, t14293: F, t14296: F, t1652: F, t27: F, t29: F, t14327: F, t558: F, t3851: F, t75201: F, t7782: F) -> (F, F, F, F, F, F, F) {
    let t76258 = t40928 * t649 * t8976;
    let t76262 = t35960 * t649 * t8947;
    let t76264 = t14293 * t11704;
    let t76268 = t14296 * t27 * t29 * t1652;
    let t76270 = t14327 * t558;
    let t76271 = t3851 * t76270;
    let t76273 = t7782 * t75201;
    (t76258, t76262, t76264, t76268, t76270, t76271, t76273)
}
