//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 64/1105 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk64<F: Float>(t123: F, t126: F, t129: F, t136: F) -> (F, F, F) {
    let t177 = 0.51785e1 * t126 + 0.905775e0 * t123 + 0.1100325e0 * t129 + 0.1241775e0 * t136;
    let t180 = 1.0 + 0.29608749977793437516e2 / t177;
    let t181 = f64::ln(t180);
    (t177, t180, t181)
}
