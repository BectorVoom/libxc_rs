//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1007/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1007<F: Float>(t77580: F, t637: F, t8635: F, t71007: F, t75282: F, t75285: F, t69710: F, t14683: F, t38530: F, t69722: F, t69728: F, t14438: F, t2868: F) -> (F, F, F, F, F, F, F, F, F) {
    let t77581 = F::cast_from(0.6818665413561335432e-1_f64) * t77580;
    let t77582 = t637 * t8635;
    let t77583 = t71007 * t77582;
    let t77584 = F::cast_from(0.27274661654245341728e-1_f64) * t77583;
    let t77585 = F::cast_from(0.30487649791575028312e-3_f64) * t75282;
    let t77586 = F::cast_from(0.40911992481368012595e-1_f64) * t75285;
    let t77587 = F::cast_from(0.79828278012425390427e-1_f64) * t69710;
    let t77588 = t38530 * t14683;
    let t77589 = F::cast_from(0.42564599893297839398e-5_f64) * t77588;
    let t77590 = F::cast_from(0.30487649791575028312e-3_f64) * t69722;
    let t77591 = F::cast_from(0.30487649791575028312e-3_f64) * t69728;
    let t77592 = t2868 * t14438;
    (t77581, t77584, t77585, t77586, t77587, t77589, t77590, t77591, t77592)
}
