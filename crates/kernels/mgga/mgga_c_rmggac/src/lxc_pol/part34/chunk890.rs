//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 890/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk890<F: Float>(t118: F, t305: F, t326: F, t333: F, t5266: F, t72088: F, t76477: F, t77233: F, t77638: F, t77816: F, t77999: F, t78228: F, t78237: F, t78240: F, t78245: F, t78247: F, t78249: F, t78251: F, t78253: F) -> (F,) {
    let t78254 = 0.49700494569958178264e-1 * t76477 + t78228 - t72088 - 0.39914139006212695214e-1 * t118 * t77638 - 0.59871208509319042821e-1 * t326 * t77233 + 0.59871208509319042821e-1 * t305 * t77816 + t78237 - t78240 + 0.11974241701863808564e0 * t5266 * t77999 * t333 + t78245 - t78247 - t78249 - t78251 - t78253;
    (t78254,)
}
