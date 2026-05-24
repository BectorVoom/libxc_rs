//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1060/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1060<F: Float>(t71033: F, t73266: F, t74647: F, t74655: F, t74662: F, t74667: F, t74670: F, t74677: F, t77134: F, t77135: F, t77137: F, t77138: F, t77143: F, t77148: F, t77150: F, t77154: F, t77155: F) -> F {
    let t80146 = t73266 + F::cast_from(0.58171619854173713844e-5_f64) * t74647 + t77134 - t77135 + F::cast_from(0.43798265232253417968e-6_f64) * t74655 + t77137 + t77138 - t71033 + F::cast_from(0.35038612185802734374e-6_f64) * t74662 + F::cast_from(0.87596530464506835935e-6_f64) * t74667 - t74670 - t77143 + F::cast_from(0.76860658247009135562e-5_f64) * t74677 - t77148 + t77150 + t77154 - t77155;
    t80146
}
