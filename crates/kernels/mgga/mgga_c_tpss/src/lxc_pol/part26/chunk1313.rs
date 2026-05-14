//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1313/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1313<F: Float>(t14247: F, t5559: F, t14258: F, t17960: F, t4761: F, t4766: F, t14300: F, t5552: F, t4718: F, t61057: F, t14242: F, t61080: F, t61087: F, t63967: F, t63974: F, t63978: F, t63991: F) -> (F,) {
    let t69989 = t5559 * t14247;
    let t69991 = t5559 * t14258;
    let t69993 = t17960 * t4761;
    let t69995 = t17960 * t4766;
    let t69997 = t5552 * t14300;
    let t69999 = t61057 * t4718;
    let t70001 = t5559 * t14242;
    let t70004 = 5.0 / 192.0 * t69989 + 5.0 / 384.0 * t69991 + 7.0 / 2304.0 * t69993 + 7.0 / 2304.0 * t69995 - t69997 / 1536.0 - 7.0 / 1152.0 * t69999 - 5.0 / 64.0 * t70001 - t61080 + t63967 + t63974 - t63978 - t63991 - 119.0 / 1728.0 * t61087;
    (t70004,)
}
