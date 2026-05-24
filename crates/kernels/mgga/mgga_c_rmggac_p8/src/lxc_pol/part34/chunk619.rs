//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 619/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk619<F: Float>(t15540: F, t14444: F, t570: F, t8940: F, t15094: F, t15130: F, t2471: F, t326: F, t650: F, t15132: F, t15134: F, t15138: F) -> (F, F, F, F, F, F, F, F, F) {
    let t15541 = F::cast_from(0.44903406381989282115e-1_f64) * t15540;
    let t15542 = t14444 * t570;
    let t15544 = F::cast_from(0.11974241701863808564e0_f64) * t8940 * t15542;
    let t15545 = F::cast_from(0.14967802127329760705e-1_f64) * t15094;
    let t15546 = F::cast_from(0.68186654135613354325e-2_f64) * t15130;
    let t15547 = t326 * t2471;
    let t15548 = t15547 * t650;
    let t15549 = F::cast_from(0.34093327067806677161e-2_f64) * t15548;
    let t15550 = F::cast_from(0.20455996240684006296e-1_f64) * t15132;
    let t15551 = F::cast_from(0.40911992481368012592e-1_f64) * t15134;
    let t15552 = F::cast_from(0.10227998120342003148e-1_f64) * t15138;
    (t15541, t15544, t15545, t15546, t15547, t15549, t15550, t15551, t15552)
}
