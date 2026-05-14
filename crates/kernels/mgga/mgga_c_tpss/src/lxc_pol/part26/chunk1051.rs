//! MGGA_C_TPSS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1051/1369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part26_v4rho3sigma_8_chunk1051<F: Float>(t10980: F, t11169: F, t11172: F, t14495: F, t14497: F, t14501: F, t14503: F, t14505: F, t14507: F, t8616: F, t8627: F, t11004: F, t11051: F, t11179: F, t11188: F, t14551: F, t14553: F, t14556: F, t14559: F, t14561: F, t14564: F, t8797: F) -> (F, F) {
    let t14610 = 0.67094444444444444443e-1 * t14495 + 0.18396666666666666667e-1 * t14497 - 0.13418888888888888889e0 * t8616 - 0.91983333333333333333e-1 * t8627 - 0.11038e0 * t14501 + 0.5519e-1 * t14503 - 0.20128333333333333333e0 * t14505 + 0.10064166666666666667e0 * t14507 - 0.26837777777777777779e0 * t10980 + t11169 + t11172;
    let t14630 = -t8797 + 0.19419375e1 * t14551 - 0.258925e1 * t14553 - 0.1294625e1 * t14556 - 0.412621875e-1 * t14559 + 0.16504875e0 * t14561 + 0.82524375e-1 * t14564 - t11179 + 0.36793333333333333333e-1 * t11051 + t11188 - 0.40256666666666666668e0 * t11004;
    (t14610, t14630)
}
