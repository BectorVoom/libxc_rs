//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 975/1102 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk975<F: Float>(t20489: F, t562: F, t1824: F, t6434: F, t20193: F, t604: F, t1453: F, t5488: F, t112: F, t22430: F, t12020: F, t225: F, t22723: F, t22891: F, t117: F, t5247: F, t6559: F) -> (F, F, F, F, F, F, F, F) {
    let t74967 = t562 * t20489;
    let t75026 = t6434 * t1824;
    let t75284 = t20193 * t604;
    let t75603 = t1453 * t5488;
    let t75784 = t22430 * t112;
    let t80640 = t225 * t12020;
    let t80670 = t22723 * t22891;
    let t80681 = t6559 * t5247 * t117;
    (t74967, t75026, t75284, t75603, t75784, t80640, t80670, t80681)
}
