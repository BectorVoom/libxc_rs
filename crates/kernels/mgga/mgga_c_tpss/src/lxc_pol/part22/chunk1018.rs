//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1018/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1018<F: Float>(t1081: F, t12135: F, t11875: F, t11942: F, t11873: F, t11880: F, t11885: F, t11890: F, t11896: F, t11899: F, t11904: F, t11908: F, t11938: F, t11952: F, t9221: F, t9223: F, t9226: F, t9228: F, t9477: F) -> (F, F) {
    let t12136 = t12135 * t1081;
    let t12145 = 0.2283111111111111111e-1 * t11875;
    let t12146 = 0.11415555555555555555e-1 * t11942;
    let t12155 = -t9477 + 0.1522074074074074074e-1 * t9221 + 0.38051851851851851851e-2 * t9223 - 0.11415555555555555555e-1 * t9226 - 0.57077777777777777777e-2 * t9228 + 0.76103703703703703702e-2 * t11938 + 0.76103703703703703701e-2 * t11873 - t12145 - t12146 + 0.19025925925925925925e-1 * t11880 - 0.68493333333333333331e-1 * t11885 - 0.2283111111111111111e-1 * t11890 - 0.11415555555555555555e-1 * t11896 + 0.10274e0 * t11899 + 0.68493333333333333332e-1 * t11904 + 0.34246666666666666666e-1 * t11908 + 0.17123333333333333333e-1 * t11952;
    (t12136, t12155)
}
