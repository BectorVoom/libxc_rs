//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 561/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk561<F: Float>(t15319: F, t15323: F, t15326: F, t15331: F, t15334: F, t15337: F, t15342: F, t15345: F, t15348: F, t15351: F, t15354: F, t15357: F, t15359: F, t15364: F, t15368: F, t15372: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15644 = 0.3830813990396805546e-4 * t15319;
    let t15645 = 0.1276937996798935182e-4 * t15323;
    let t15646 = 0.1276937996798935182e-4 * t15326;
    let t15647 = 0.58171619854173713846e-5 * t15331;
    let t15648 = 0.2627895913935205078e-5 * t15334;
    let t15649 = 0.87596530464506835935e-6 * t15337;
    let t15650 = 0.87596530464506835935e-6 * t15342;
    let t15651 = 0.17519306092901367188e-6 * t15345;
    let t15652 = 0.4379826523225341797e-6 * t15348;
    let t15653 = 0.35038612185802734376e-6 * t15351;
    let t15654 = 0.52557918278704101564e-6 * t15354;
    let t15655 = 0.52557918278704101564e-6 * t15357;
    let t15656 = 0.14967802127329760705e-1 * t15359;
    let t15657 = 0.58171619854173713846e-5 * t15364;
    let t15658 = 0.17451485956252114154e-4 * t15368;
    let t15660 = 0.23268647941669485538e-4 * t15372;
    (t15644, t15645, t15646, t15647, t15648, t15649, t15650, t15651, t15652, t15653, t15654, t15655, t15656, t15657, t15658, t15660)
}
