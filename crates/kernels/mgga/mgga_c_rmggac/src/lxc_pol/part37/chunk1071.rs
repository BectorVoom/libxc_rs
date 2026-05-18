//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1071/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1071<F: Float>(t69105: F, t69107: F, t71319: F, t75029: F, t75040: F, t75045: F, t75048: F, t75054: F, t77425: F, t77426: F, t77427: F, t77428: F, t77430: F, t77431: F, t77437: F, t77439: F, t77441: F) -> F {
    let t80210 = -t77425 - t77426 + t77427 - t77428 + F::new(0.87596530464506835932e-6) * t75029 + t77430 - t77431 + F::new(0.17519306092901367186e-5) * t75040 + F::new(0.72714524817717142305e-5) * t75045 - t77437 - F::new(0.58171619854173713844e-5) * t75048 - t77439 - F::new(0.17519306092901367186e-5) * t75054 - t71319 - t69105 - t69107 - t77441;
    t80210
}
