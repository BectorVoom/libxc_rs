//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1203/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1203<F: Float>(t19046: F, t7338: F, t6169: F, t7344: F, t18375: F, t7339: F, t27599: F, t4997: F, t18329: F, t7310: F, t18324: F, t18371: F, t24741: F, t19031: F, t2139: F, t471: F) -> (F, F, F, F, F, F, F, F) {
    let t104015 = t19046 * t7338;
    let t104018 = t6169 * t7344;
    let t104048 = t7339 * t18375;
    let t104050 = t27599 * t4997;
    let t104085 = t7310 * t18329;
    let t104088 = t7310 * t18324;
    let t104094 = t24741 * t18371;
    let t104107 = t471 * t2139 * t19031;
    (t104015, t104018, t104048, t104050, t104085, t104088, t104094, t104107)
}
