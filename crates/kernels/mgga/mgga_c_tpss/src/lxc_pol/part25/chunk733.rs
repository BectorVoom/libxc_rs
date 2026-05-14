//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 733/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk733<F: Float>(t1081: F, t5161: F, t2981: F, t2988: F, t4044: F, t4093: F, t5066: F, t5070: F, t5074: F, t5086: F, t5093: F, t5099: F, t5101: F, t5105: F, t5108: F, t5111: F) -> (F, F) {
    let t5162 = t5161 * t1081;
    let t5177 = -0.1294625e1 * t5086 + 0.258925e1 * t5093 + t2981 - 0.20128333333333333334e0 * t4044 - 0.20128333333333333333e0 * t5066 + 0.60385e0 * t5070 + 0.301925e0 * t5074 + 0.82524375e-1 * t5099 + 0.16504875e0 * t5101 + t2988 - 0.11038e0 * t4093 - 0.27595e-1 * t5105 + 0.16557e0 * t5108 + 0.82785e-1 * t5111;
    (t5162, t5177)
}
