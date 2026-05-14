//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 453/930 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk453<F: Float>(t1191: F, t225: F, t1202: F, t1226: F, t3258: F, t3261: F, t3268: F, t3310: F, t3318: F, t3408: F, t3410: F, t3413: F, t3417: F, t3421: F, t3425: F, t475: F) -> (F, F, F, F) {
    let t3487 = t1191 * t225;
    let t3490 = t1202 * t1226;
    let t3493 = -t3258 + t3261 - t3268 + t3310 + t3318 + t3408 + t3410 - t3413 + t3417 - t3421 - t3425;
    let t3494 = t3493 * t475;
    (t3487, t3490, t3493, t3494)
}
