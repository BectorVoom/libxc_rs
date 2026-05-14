//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 928/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk928<F: Float>(t8550: F, t8553: F, t8557: F, t2724: F, t345: F, t2716: F, t941: F, t2662: F, t921: F, t2668: F, t917: F, t2473: F, t845: F, t2530: F, t841: F, t2529: F, t281: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8559 = t8550 * t8553 * t8557;
    let t8561 = t2724 * t345;
    let t8568 = t8550 * t2716 * t8557;
    let t8577 = t8550 * t941 * t8557;
    let t8586 = t2662 * t921;
    let t8588 = t917 * t2668;
    let t8590 = t2473 * t845;
    let t8595 = t841 * t2530;
    let t8599 = 1.0 / t2529 / t281;
    (t8559, t8561, t8568, t8577, t8586, t8588, t8590, t8595, t8599)
}
