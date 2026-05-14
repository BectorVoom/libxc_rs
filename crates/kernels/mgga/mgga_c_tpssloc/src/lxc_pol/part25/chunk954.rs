//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 954/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk954<F: Float>(t23095: F, t23105: F, t23107: F, t23140: F, t23143: F, t23100: F, t23114: F, t23117: F, t23119: F, t23125: F, t23128: F, t23130: F, t23134: F, t23136: F, t23147: F, t24217: F) -> (F,) {
    let t24218 = 0.10541775202358879834e-2 * t23095;
    let t24220 = 0.33643963411783659044e-4 * t23105;
    let t24221 = 119.0 / 3456.0 * t23107;
    let t24230 = 0.22608743412718618878e-1 * t23140;
    let t24231 = 35.0 / 216.0 * t23143;
    let t24233 = t24218 + 0.48447307312968469024e-2 * t23100 - t24220 + t24221 + 0.13457585364713463618e-3 * t23114 + t23117 / 768.0 - 7.0 / 576.0 * t23119 + 0.80745512188280781706e-3 * t23125 - t23128 / 96.0 + 5.0 / 192.0 * t23130 + 7.0 / 144.0 * t23134 - t23136 / 192.0 + t24230 + t24231 + t23147 / 96.0;
    let t24234 = t24217 + t24233;
    (t24234,)
}
