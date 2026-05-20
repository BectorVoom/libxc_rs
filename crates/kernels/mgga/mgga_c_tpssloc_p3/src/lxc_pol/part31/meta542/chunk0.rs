//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1763/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1763<F: Float>(t6892: F, t81186: F, t1987: F, t81144: F, t9537: F, t107: F, t835: F, t240: F, t656: F, t666: F, t2331: F, t625: F) -> (F, F, F, F, F, F) {
    let t81375 = t81186 * t6892;
    let t81398 = t81144 * t9537 * t1987;
    let t81437 = t835 * t107;
    let t81439 = t240 * t656;
    let t81440 = t81439 * t666;
    let t81442 = t625 * t2331;
    (t81375, t81398, t81437, t81439, t81440, t81442)
}
