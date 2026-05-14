//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1063/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1063<F: Float>(t28107: F, t553: F, t1998: F, t6434: F, t214: F, t1985: F, t19739: F, t550: F, t6976: F, t1992: F, t19660: F, t22709: F, t6388: F, t6420: F, t6987: F, t1825: F, t26458: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28156 = t553 * t28107;
    let t28159 = t1998 * t6434;
    let t28160 = t214 * t28159;
    let t28161 = t1985 * t28160;
    let t28163 = t19739 * t550;
    let t28164 = t6976 * t28163;
    let t28165 = t1992 * t28164;
    let t28167 = t19660 * t550;
    let t28168 = t6976 * t28167;
    let t28169 = t1992 * t28168;
    let t28171 = t22709 * t6388;
    let t28174 = t6987 * t6420;
    let t28178 = t26458 * t1825;
    (t28156, t28159, t28160, t28161, t28163, t28164, t28165, t28167, t28168, t28169, t28171, t28174, t28178)
}
