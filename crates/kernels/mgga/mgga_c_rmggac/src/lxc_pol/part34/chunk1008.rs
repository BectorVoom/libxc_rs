//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1008/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1008<F: Float>(t77592: F, t14498: F, t5928: F, t15526: F, t2604: F, t69745: F, t71448: F, t75440: F, t77573: F, t77575: F, t77578: F, t77581: F, t77584: F, t77585: F, t77586: F, t77587: F, t77589: F, t77590: F, t77591: F) -> F {
    let t77593 = F::new(0.14967802127329760705e-1) * t77592;
    let t77595 = F::new(0.39914139006212695214e-1) * t5928 * t14498;
    let t77596 = t2604 * t15526;
    let t77597 = F::new(0.14967802127329760705e-1) * t77596;
    let t77598 = F::new(0.16263363996404810741e-4) * t69745;
    let t77599 = t77573 + t77575 + t77578 - t77581 - t77584 - t77585 - t77586 + t71448 - t77587 + t77589 - t77590 - t77591 + t77593 + t77595 - t75440 + t77597 + t77598;
    t77599
}
