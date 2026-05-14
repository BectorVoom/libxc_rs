//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 504/1266 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk504<F: Float>(t2061: F, t94: F, t1163: F, t645: F, t112: F, t2023: F, t600: F, t641: F, t111: F, t629: F) -> (F, F, F, F, F) {
    let t2062 = t94 * t2061;
    let t2065 = t1163 * t645;
    let t2069 = 11.0 / 9.0 * t2023 * t112;
    let t2070 = t600 * t641;
    let t2073 = 1.0 / t629 / t111;
    (t2062, t2065, t2069, t2070, t2073)
}
