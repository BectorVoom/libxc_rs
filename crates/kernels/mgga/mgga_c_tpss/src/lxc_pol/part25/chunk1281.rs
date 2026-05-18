//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1281/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1281<F: Float>(t198: F, t206: F, t5848: F, t5831: F, t768: F, t61024: F, t61079: F, t1811: F, t31814: F, t8096: F, t61868: F, t507: F, t5935: F) -> (F, F, F, F, F, F, F, F) {
    let t62610 = t198 * t206 * t5848;
    let t62671 = t768 * t5831;
    let t62690 = F::new(595.0) / F::new(2592.0) * t61024;
    let t62711 = F::new(455.0) / F::new(648.0) * t61079;
    let t62807 = t1811 * t31814;
    let t62829 = t5848 * t8096;
    let t63006 = F::new(308.0) / F::new(27.0) * t61868;
    let t63042 = t507 * t5935;
    (t62610, t62671, t62690, t62711, t62807, t62829, t63006, t63042)
}
