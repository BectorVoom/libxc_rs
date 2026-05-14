//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 535/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk535<F: Float>(t2169: F, t785: F, t236: F, t339: F, t769: F, t72: F, t799: F, t240: F) -> (F, F, F, F) {
    let t2170 = t2169 * t785;
    let t2173 = t339 * t769 * t236;
    let t2174 = t799 * t72;
    let t2175 = t2174 * t240;
    (t2170, t2173, t2174, t2175)
}
