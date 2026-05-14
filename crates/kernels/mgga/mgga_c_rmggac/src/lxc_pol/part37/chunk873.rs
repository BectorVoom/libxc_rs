//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 873/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk873<F: Float>(t14182: F, t14655: F, t14656: F, t14659: F, t14660: F, t14661: F, t14662: F, t14663: F, t14665: F, t14670: F, t14674: F, t15525: F, t15528: F, t15529: F, t15905: F, t73691: F, t73693: F, t73696: F, t73708: F, t73714: F, t73719: F, t73724: F, t73729: F, t73734: F, t73739: F, t76604: F, t76607: F, t76608: F, t76613: F, t76617: F, t76619: F) -> (F, F) {
    let t79961 = -t15525 - t14655 - t14656 + t14182 + t14659 - t14660 + t14661 + t14662 - t14663 - t14665 + t14670 - t14674 - t15905 + t15528 + t15529;
    let t79971 = -t76604 - t73691 - 0.58171619854173713844e-5 * t73693 + 0.58171619854173713844e-5 * t73696 - t76607 - t76608 - t76613 + t76617 + t76619 - 0.87596530464506835932e-6 * t73708 - 0.35038612185802734374e-6 * t73714 - 0.43798265232253417968e-6 * t73719 - 0.35038612185802734374e-6 * t73724 + 0.52557918278704101561e-6 * t73729 - 0.52557918278704101561e-6 * t73734 - 0.17519306092901367187e-6 * t73739;
    (t79961, t79971)
}
