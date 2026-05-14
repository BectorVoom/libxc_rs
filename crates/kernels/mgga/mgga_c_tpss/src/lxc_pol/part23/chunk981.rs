//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 981/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk981<F: Float>(t10681: F, t10682: F, t10703: F, t10732: F, t219: F, t73: F, t776: F, t1364: F, t2387: F, t2116: F, t3610: F, t799: F, t750: F, t2133: F, t3657: F, t10552: F, t778: F) -> (F, F, F, F, F, F) {
    let t10735 = (t10681 + t10682 + t10703 + t10732) * t219;
    let t10745 = t776 * t73;
    let t10750 = t2387 * t1364;
    let t10751 = t10750 * t2116;
    let t10754 = t799 * t3610;
    let t10755 = t10754 * t750;
    let t10758 = t3657 * t2133;
    let t10761 = t778 * t10552;
    (t10735, t10745, t10751, t10755, t10758, t10761)
}
