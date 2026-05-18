//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 629/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk629<F: Float>(t15636: F, t15303: F, t15307: F, t15311: F, t15315: F, t15319: F, t15323: F, t15326: F, t15331: F, t15334: F, t15337: F, t15342: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t15637 = F::new(0.68186654135613354322e-2) * t15636;
    let t15638 = F::new(0.93188427318671584245e-2) * t15303;
    let t15639 = F::new(0.15531404553111930708e-1) * t15307;
    let t15640 = F::new(0.10227998120342003148e-1) * t15311;
    let t15643 = F::new(0.40911992481368012592e-1) * t15315;
    let t15644 = F::new(0.3830813990396805546e-4) * t15319;
    let t15645 = F::new(0.1276937996798935182e-4) * t15323;
    let t15646 = F::new(0.1276937996798935182e-4) * t15326;
    let t15647 = F::new(0.58171619854173713846e-5) * t15331;
    let t15648 = F::new(0.2627895913935205078e-5) * t15334;
    let t15649 = F::new(0.87596530464506835935e-6) * t15337;
    let t15650 = F::new(0.87596530464506835935e-6) * t15342;
    (t15637, t15638, t15639, t15640, t15643, t15644, t15645, t15646, t15647, t15648, t15649, t15650)
}
