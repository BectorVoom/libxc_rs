//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 617/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk617<F: Float>(t15130: F, t2471: F, t326: F, t650: F, t15132: F, t15134: F, t15138: F, t118: F, t15530: F, t15164: F, t15167: F, t15170: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t15546 = F::new(0.68186654135613354325e-2) * t15130;
    let t15547 = t326 * t2471;
    let t15548 = t15547 * t650;
    let t15549 = F::new(0.34093327067806677161e-2) * t15548;
    let t15550 = F::new(0.20455996240684006296e-1) * t15132;
    let t15551 = F::new(0.40911992481368012592e-1) * t15134;
    let t15552 = F::new(0.10227998120342003148e-1) * t15138;
    let t15557 = F::new(0.39914139006212695214e-1) * t118 * t15530;
    let t15559 = F::new(0.20455996240684006298e-1) * t15164;
    let t15560 = F::new(0.2727466165424534173e-1) * t15167;
    let t15561 = F::new(0.13637330827122670865e-1) * t15170;
    (t15546, t15547, t15549, t15550, t15551, t15552, t15557, t15559, t15560, t15561)
}
