//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 930/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk930<F: Float>(t76618: F, t73691: F, t73693: F, t73696: F, t73708: F, t73714: F, t73719: F, t73724: F, t73729: F, t73734: F, t73739: F, t76604: F, t76607: F, t76608: F, t76613: F, t76617: F) -> F {
    let t76619 = F::new(0.99317399751028291929e-5) * t76618;
    let t76627 = -t76604 - t73691 - F::new(0.58171619854173713846e-5) * t73693 + F::new(0.58171619854173713846e-5) * t73696 - t76607 - t76608 - t76613 + t76617 + t76619 - F::new(0.87596530464506835935e-6) * t73708 - F::new(0.35038612185802734376e-6) * t73714 - F::new(0.4379826523225341797e-6) * t73719 - F::new(0.35038612185802734376e-6) * t73724 + F::new(0.52557918278704101564e-6) * t73729 - F::new(0.52557918278704101564e-6) * t73734 - F::new(0.17519306092901367188e-6) * t73739;
    t76627
}
