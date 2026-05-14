//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1242/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1242<F: Float>(t63935: F, t63945: F, t63949: F, t63957: F, t63964: F, t66420: F, t69972: F, t69974: F, t69976: F, t69978: F, t69981: F, t69983: F, t69985: F, t62711: F, t63998: F, t66423: F, t66427: F, t66429: F, t66434: F, t69989: F, t69991: F, t69993: F, t69995: F, t69997: F, t69999: F, t70001: F) -> (F, F) {
    let t72069 = -t69972 / 24.0 + t69974 / 96.0 + t69976 / 96.0 - t69978 / 96.0 - t63935 - 7.0 / 24.0 * t69981 + 7.0 / 72.0 * t69983 + t69985 / 192.0 - 119.0 / 1728.0 * t63945 - t63949 - 35.0 / 54.0 * t63957 + t66420 - 119.0 / 432.0 * t63964;
    let t72077 = 5.0 / 96.0 * t69989 + 5.0 / 192.0 * t69991 + 7.0 / 1152.0 * t69993 + 7.0 / 1152.0 * t69995 - t69997 / 768.0 - 7.0 / 576.0 * t69999 - 5.0 / 32.0 * t70001 - t62711 + t66423 + t66427 - t66429 - t66434 - t63998;
    (t72069, t72077)
}
