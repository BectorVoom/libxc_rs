//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1102/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1102<F: Float>(t225: F, t22643: F, t1987: F, t81144: F, t9537: F, t107: F, t835: F, t240: F, t656: F, t2331: F, t625: F, t63: F, t9365: F) -> (F, F, F, F, F, F) {
    let t81326 = t22643 * t225;
    let t81398 = t81144 * t9537 * t1987;
    let t81437 = t835 * t107;
    let t81439 = t240 * t656;
    let t81442 = t625 * t2331;
    let t81446 = t63 * t9365;
    (t81326, t81398, t81437, t81439, t81442, t81446)
}
