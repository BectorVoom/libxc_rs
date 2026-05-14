//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 806/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk806<F: Float>(t5709: F, t5909: F, t5714: F, t5724: F, t5717: F, t5722: F, t5729: F) -> (F, F, F, F) {
    let t5910 = t5909 * t5709;
    let t5913 = 7.0 / 144.0 * t5714;
    let t5916 = 7.0 / 1152.0 * t5724;
    let t5918 = -t5913 - t5717 / 24.0 - t5722 / 768.0 - t5916 - t5729 / 192.0;
    (t5910, t5913, t5916, t5918)
}
