//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 826/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk826<F: Float>(t77572: F, t15504: F, t16043: F, t637: F, t8625: F, t71163: F, t8631: F, t72142: F, t8635: F, t71007: F, t75282: F, t75285: F, t69710: F, t14683: F, t38530: F, t69722: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t77573 = 0.53205749866622299248e-5 * t77572;
    let t77574 = t16043 * t15504;
    let t77575 = 0.42564599893297839398e-5 * t77574;
    let t77576 = t637 * t8625;
    let t77577 = t71163 * t77576;
    let t77578 = 0.40911992481368012592e-1 * t77577;
    let t77579 = t637 * t8631;
    let t77580 = t72142 * t77579;
    let t77581 = 0.6818665413561335432e-1 * t77580;
    let t77582 = t637 * t8635;
    let t77583 = t71007 * t77582;
    let t77584 = 0.27274661654245341728e-1 * t77583;
    let t77585 = 0.30487649791575028312e-3 * t75282;
    let t77586 = 0.40911992481368012595e-1 * t75285;
    let t77587 = 0.79828278012425390427e-1 * t69710;
    let t77588 = t38530 * t14683;
    let t77589 = 0.42564599893297839398e-5 * t77588;
    let t77590 = 0.30487649791575028312e-3 * t69722;
    (t77573, t77575, t77578, t77581, t77584, t77585, t77586, t77587, t77589, t77590)
}
