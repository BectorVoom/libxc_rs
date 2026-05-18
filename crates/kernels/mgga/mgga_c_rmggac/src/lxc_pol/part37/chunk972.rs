//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 972/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk972<F: Float>(t69710: F, t14683: F, t38530: F, t69722: F, t69728: F, t14438: F, t2868: F, t14498: F, t5928: F, t15526: F, t2604: F, t69745: F) -> (F, F, F, F, F, F, F, F) {
    let t77587 = F::new(0.79828278012425390427e-1) * t69710;
    let t77588 = t38530 * t14683;
    let t77589 = F::new(0.42564599893297839398e-5) * t77588;
    let t77590 = F::new(0.30487649791575028312e-3) * t69722;
    let t77591 = F::new(0.30487649791575028312e-3) * t69728;
    let t77592 = t2868 * t14438;
    let t77593 = F::new(0.14967802127329760705e-1) * t77592;
    let t77595 = F::new(0.39914139006212695214e-1) * t5928 * t14498;
    let t77596 = t2604 * t15526;
    let t77597 = F::new(0.14967802127329760705e-1) * t77596;
    let t77598 = F::new(0.16263363996404810741e-4) * t69745;
    (t77587, t77589, t77590, t77591, t77593, t77595, t77597, t77598)
}
