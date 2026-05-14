//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1105/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1105<F: Float>(t20193: F, t604: F, t1453: F, t5488: F, t112: F, t22430: F, t12020: F, t225: F, t22723: F, t22891: F, t117: F, t5247: F, t6559: F, t22684: F, t6546: F, t131: F, t1365: F) -> (F, F, F, F, F, F, F, F) {
    let t75284 = t20193 * t604;
    let t75603 = t1453 * t5488;
    let t75784 = t22430 * t112;
    let t80640 = t225 * t12020;
    let t80670 = t22723 * t22891;
    let t80681 = t6559 * t5247 * t117;
    let t80727 = t6546 * t22684;
    let t80730 = t1365 * t131;
    (t75284, t75603, t75784, t80640, t80670, t80681, t80727, t80730)
}
