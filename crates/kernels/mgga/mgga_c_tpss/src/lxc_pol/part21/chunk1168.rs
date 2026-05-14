//! MGGA_C_TPSS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1168/1368 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part21_v4rho3sigma_3_chunk1168<F: Float>(t18101: F, t18129: F, t219: F, t5624: F, t1705: F, t2768: F, t935: F, t5570: F, t5628: F, t5638: F, t347: F, t9066: F, t1726: F, t2777: F, t2775: F, t5637: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18130 = t18101 + t18129;
    let t18131 = param_beta * t18130;
    let t18133 = t5624 * t219;
    let t18139 = t1705 * t2768;
    let t18140 = t18139 * t935;
    let t18142 = t5628 * t5570;
    let t18145 = t5628 * t5638;
    let t18150 = t9066 * t347;
    let t18152 = t18150 * t1726 * t2777;
    let t18155 = t5637 * t2775;
    (t18130, t18131, t18133, t18139, t18140, t18142, t18145, t18150, t18152, t18155)
}
