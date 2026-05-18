//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 761/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk761<F: Float>(t1062: F, t5129: F, t2937: F, t2944: F, t4044: F, t4093: F, t5066: F, t5070: F, t5074: F, t5086: F, t5093: F, t5099: F, t5101: F, t5105: F, t5108: F, t5111: F) -> (F, F) {
    let t5130 = t5129 * t1062;
    let t5145 = -F::new(0.17648625e1) * t5086 + F::new(0.3529725e1) * t5093 + t2937 - F::new(0.34431666666666666666e0) * t4044 - F::new(0.34431666666666666667e0) * t5066 + F::new(0.103295e1) * t5070 + F::new(0.516475e0) * t5074 + F::new(0.31558125e0) * t5099 + F::new(0.6311625e0) * t5101 + t2944 - F::new(0.13892666666666666667e0) * t4093 - F::new(0.34731666666666666667e-1) * t5105 + F::new(0.20839e0) * t5108 + F::new(0.104195e0) * t5111;
    (t5130, t5145)
}
