//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1048/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1048<F: Float>(t14632: F, t904: F, t4886: F, t876: F, t10980: F, t11309: F, t11312: F, t14495: F, t14497: F, t14501: F, t14503: F, t14505: F, t14507: F, t8616: F, t8627: F, t11004: F, t11051: F, t11319: F, t11328: F, t14551: F, t14553: F, t14556: F, t14559: F, t14561: F, t14564: F, t8872: F) -> (F, F, F, F) {
    let t14734 = t14632 * t904;
    let t14739 = t4886 * t876;
    let t14770 = 0.11477222222222222222e0 * t14495 + 0.23154444444444444445e-1 * t14497 - 0.22954444444444444444e0 * t8616 - 0.11577222222222222222e0 * t8627 - 0.13892666666666666667e0 * t14501 + 0.69463333333333333333e-1 * t14503 - 0.34431666666666666667e0 * t14505 + 0.17215833333333333333e0 * t14507 - 0.45908888888888888888e0 * t10980 + t11309 + t11312;
    let t14790 = -t8872 + 0.264729375e1 * t14551 - 0.3529725e1 * t14553 - 0.17648625e1 * t14556 - 0.157790625e0 * t14559 + 0.6311625e0 * t14561 + 0.31558125e0 * t14564 - t11319 + 0.4630888888888888889e-1 * t11051 + t11328 - 0.68863333333333333332e0 * t11004;
    (t14734, t14739, t14770, t14790)
}
