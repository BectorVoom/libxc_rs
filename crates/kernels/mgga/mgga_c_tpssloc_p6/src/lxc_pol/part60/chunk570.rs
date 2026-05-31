//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 570/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk570<F: Float>(t1176: F, t461: F, t491: F, t225: F, t497: F, t457: F, t1240: F, t1193: F, t2127: F, t210: F, t2120: F, t2132: F, t52: F) -> (F, F, F, F, F, F, F, F, F) {
    let t7284 = t1176 * t461;
    let t7285 = t7284 * t491;
    let t7286 = t225 * t497;
    let t7299 = t457 * t461;
    let t7300 = t7299 * t491;
    let t7301 = t225 * t1240;
    let t7309 = t2127 * t1193 / F::cast_from(288.0_f64);
    let t7310 = t2120 * t210;
    let t7313 = t2132 * t52;
    (t7284, t7285, t7286, t7299, t7300, t7301, t7309, t7310, t7313)
}
