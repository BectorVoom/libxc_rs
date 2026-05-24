//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1040/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1040<F: Float>(t73795: F, t73801: F, t76648: F, t76652: F, t76656: F, t76658: F, t76662: F, t76666: F, t76668: F, t76670: F, t76671: F, t76673: F, t76674: F, t76679: F, t76682: F, t76683: F, t76684: F) -> F {
    let t79980 = -t76648 - t76652 - t76656 + t76658 + t76662 + t76666 + t76668 - t76670 + t76671 + F::cast_from(0.87596530464506835932e-6_f64) * t73795 - t76673 + t76674 + F::cast_from(0.87596530464506835932e-6_f64) * t73801 - t76679 + t76682 - t76683 - t76684;
    t79980
}
