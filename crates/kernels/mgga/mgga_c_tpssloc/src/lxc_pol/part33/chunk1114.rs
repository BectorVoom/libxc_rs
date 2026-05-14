//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1114/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1114<F: Float>(t81398: F, t107: F, t835: F, t240: F, t656: F, t2331: F, t625: F, t63: F, t9365: F, t22641: F, t9523: F, t1887: F, t23069: F, t6561: F, t80741: F, t6643: F) -> (F, F, F, F, F, F, F, F, F) {
    let t81399 = 0.13707783890401886971e-2 * t81398;
    let t81437 = t835 * t107;
    let t81438 = 154.0 / 27.0 * t81437;
    let t81439 = t240 * t656;
    let t81442 = t625 * t2331;
    let t81446 = t63 * t9365;
    let t81573 = t22641 * t9523;
    let t81591 = t23069 * t1887;
    let t81597 = t80741 * t6561;
    let t81598 = t81597 * t6643;
    (t81399, t81438, t81439, t81442, t81446, t81573, t81591, t81597, t81598)
}
