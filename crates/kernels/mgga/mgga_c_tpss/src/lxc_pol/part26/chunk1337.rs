//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1337/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1337<F: Float>(t1600: F, t1663: F, t20786: F, t20950: F, t20957: F, t20981: F, t3538: F, t4341: F, t6103: F, t6480: F, t68977: F, t68980: F, t68988: F, t68992: F, t69006: F, t69016: F, t69018: F, t69020: F, t69022: F, t69025: F, t69028: F, t69030: F, t69372: F, t69373: F) -> (F,) {
    let t72840 = -2.0 * t1600 * t20786 + 2.0 * t1663 * t20981 - 4.0 * t20950 * t6103 - 4.0 * t20957 * t3538 - 2.0 * t4341 * t6480 + t68977 + t68980 + t68988 + t68992 - t69006 - t69016 - t69018 - t69020 - t69022 - t69025 - t69028 - t69030 + t69372 - t69373;
    (t72840,)
}
