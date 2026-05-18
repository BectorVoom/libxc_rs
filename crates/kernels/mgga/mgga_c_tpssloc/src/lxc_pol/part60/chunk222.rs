//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 222/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk222<F: Float>(t1222: F, t485: F, t372: F, t483: F, t479: F, t471: F, t404: F, t415: F, t61: F, t225: F, t492: F) -> (F, F, F, F, F, F, F) {
    let t1224 = t485 * t1222 / F::new(4608.0);
    let t1225 = t483 * t372;
    let t1226 = t479 * t1225;
    let t1227 = t471 * t1226;
    let t1229 = F::new(1.0) / t415 / t404;
    let t1230 = t61 * t1229;
    let t1238 = t492 * t225;
    (t1224, t1225, t1226, t1227, t1229, t1230, t1238)
}
