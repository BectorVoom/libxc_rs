//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 915/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk915<F: Float>(t69835: F, t69838: F, t71505: F, t75524: F, t75531: F, t75539: F, t75545: F, t75550: F, t75553: F, t75556: F, t75558: F, t77646: F, t77653: F, t77654: F, t77658: F, t77659: F, t77660: F) -> (F,) {
    let t80263 = -t77646 - 0.40878380883436523435e-5 * t75524 + t71505 - t69835 - t69838 - t75531 - t77653 + t77654 + t75539 + t75545 + t75550 - 0.35038612185802734374e-6 * t75553 + 0.35038612185802734374e-6 * t75556 - 0.87257429781260570765e-4 * t75558 - t77658 - t77659 + t77660;
    (t80263,)
}
