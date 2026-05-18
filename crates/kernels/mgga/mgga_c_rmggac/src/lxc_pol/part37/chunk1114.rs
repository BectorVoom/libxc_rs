//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1114/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1114<F: Float>(t76477: F, t118: F, t72088: F, t76476: F, t78222: F, t78225: F, t78228: F, t78237: F, t78240: F, t78245: F, t78247: F, t78249: F, t78251: F, t78253: F, t80163: F) -> F {
    let t80496 = F::new(0.49700494569958178262e-1) * t76477;
    let t80497 = -F::new(0.39914139006212695214e-1) * t118 * t80163 - t76476 - t78222 + t78225 + t80496 + t78228 - t72088 + t78237 - t78240 + t78245 - t78247 - t78249 - t78251 - t78253;
    t80497
}
