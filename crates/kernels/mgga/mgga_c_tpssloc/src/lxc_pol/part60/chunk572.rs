//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 572/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk572<F: Float>(t1222: F, t2141: F, t1225: F, t2139: F, t471: F, t2145: F, t225: F, t1170: F, t2148: F, t2121: F, t7284: F, t477: F, t491: F) -> (F, F, F, F, F, F, F, F) {
    let t7343 = t2141 * t1222 / F::new(2304.0);
    let t7344 = t2139 * t1225;
    let t7345 = t471 * t7344;
    let t7351 = t2145 * t225;
    let t7359 = t1170 * t2148;
    let t7361 = F::new(0.27415567780803773942e-2) * t2121 * t7359;
    let t7362 = t7284 * t225;
    let t7363 = t477 * t491;
    (t7343, t7344, t7345, t7351, t7359, t7361, t7362, t7363)
}
