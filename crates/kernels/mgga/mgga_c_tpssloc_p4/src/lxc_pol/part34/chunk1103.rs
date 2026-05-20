//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1103/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1103<F: Float>(t22641: F, t9523: F, t1887: F, t23069: F, t6561: F, t80741: F, t6643: F, t2588: F, t225: F, t814: F, t22715: F, t6551: F) -> (F, F, F, F, F, F, F) {
    let t81573 = t22641 * t9523;
    let t81591 = t23069 * t1887;
    let t81597 = t80741 * t6561;
    let t81598 = t81597 * t6643;
    let t81612 = t22641 * t2588;
    let t81613 = t225 * t814;
    let t81632 = t22715 * t6551;
    (t81573, t81591, t81597, t81598, t81612, t81613, t81632)
}
