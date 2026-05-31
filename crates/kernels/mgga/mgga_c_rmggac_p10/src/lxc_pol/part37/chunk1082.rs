//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1082/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1082<F: Float>(t530: F, t73395: F, t73397: F, t75607: F, t75611: F, t75623: F, t77686: F, t77690: F, t77691: F, t77693: F, t77694: F, t77695: F, t77696: F, t77697: F, t77700: F, t77703: F, t77704: F, t77705: F) -> F {
    let t80275 = -t77686 - t75607 - F::cast_from(0.17451485956252114153e-4_f64) * t75611 - t77690 - t77691 - F::cast_from(0.17519306092901367186e-5_f64) * t75623 + t77693 - t77694 + t77695 - t77696 + t77697 + t77700 + t73395 + t77703 - F::cast_from(0.2363e1_f64) * t530 * t73397 + t77704 + t77705;
    t80275
}
