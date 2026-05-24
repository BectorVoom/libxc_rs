//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1041/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1041<F: Float>(t73807: F, t73812: F, t73817: F, t73827: F, t73833: F, t73840: F, t73843: F, t73847: F, t73849: F, t76688: F, t76689: F, t76690: F, t76693: F, t76696: F, t76701: F, t76703: F, t76708: F) -> F {
    let t79988 = -F::cast_from(0.58171619854173713844e-5_f64) * t73807 - F::cast_from(0.58171619854173713844e-5_f64) * t73812 + t76688 + t73817 - t76689 + t76690 - F::cast_from(0.43798265232253417968e-6_f64) * t73827 - F::cast_from(0.15329392831288696289e-5_f64) * t73833 - t76693 - F::cast_from(0.87596530464506835936e-6_f64) * t73840 + F::cast_from(0.1313947956967602539e-5_f64) * t73843 - t76696 - F::cast_from(0.58171619854173713844e-5_f64) * t73847 - t73849 - t76701 - t76703 - t76708;
    t79988
}
