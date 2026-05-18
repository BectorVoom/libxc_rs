//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 630/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk630<F: Float>(t15345: F, t15348: F, t15351: F, t15354: F, t15357: F, t15359: F, t15364: F, t15368: F, t15372: F, t15377: F, t15380: F, t15389: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15651 = F::new(0.17519306092901367188e-6) * t15345;
    let t15652 = F::new(0.4379826523225341797e-6) * t15348;
    let t15653 = F::new(0.35038612185802734376e-6) * t15351;
    let t15654 = F::new(0.52557918278704101564e-6) * t15354;
    let t15655 = F::new(0.52557918278704101564e-6) * t15357;
    let t15656 = F::new(0.14967802127329760705e-1) * t15359;
    let t15657 = F::new(0.58171619854173713846e-5) * t15364;
    let t15658 = F::new(0.17451485956252114154e-4) * t15368;
    let t15660 = F::new(0.23268647941669485538e-4) * t15372;
    let t15661 = F::new(0.58171619854173713846e-5) * t15377;
    let t15662 = F::new(0.58171619854173713846e-5) * t15380;
    let t15663 = F::new(0.35038612185802734376e-6) * t15389;
    (t15651, t15652, t15653, t15654, t15655, t15656, t15657, t15658, t15660, t15661, t15662, t15663)
}
