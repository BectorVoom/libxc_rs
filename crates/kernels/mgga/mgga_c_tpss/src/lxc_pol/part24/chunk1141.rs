//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1141/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk1141<F: Float>(t5628: F, t5638: F, t347: F, t9066: F, t2775: F, t5637: F, t1729: F) -> (F, F, F, F) {
    let t18145 = t5628 * t5638;
    let t18150 = t9066 * t347;
    let t18155 = t5637 * t2775;
    let t18156 = t1729 * t18155;
    (t18145, t18150, t18155, t18156)
}
