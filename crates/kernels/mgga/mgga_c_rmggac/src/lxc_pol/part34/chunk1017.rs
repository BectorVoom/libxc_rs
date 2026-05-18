//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 1017/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk1017<F: Float>(t77699: F, t75638: F, t75640: F, t75644: F, t14501: F, t1540: F, t2868: F, t3230: F, t75607: F, t75611: F, t75623: F, t77686: F, t77690: F, t77691: F, t77693: F, t77694: F, t77695: F, t77696: F, t77697: F) -> F {
    let t77700 = F::new(0.13637330827122670864e-1) * t77699;
    let t77703 = F::new(0.14967802127329760705e-1) * t75638;
    let t77704 = F::new(0.10227998120342003148e-1) * t75640;
    let t77705 = F::new(0.10227998120342003148e-1) * t75644;
    let t77706 = -t77686 - F::new(0.59871208509319042821e-1) * t2868 * t14501 - t75607 - F::new(0.17451485956252114154e-4) * t75611 - t77690 - t77691 - F::new(0.17519306092901367187e-5) * t75623 + t77693 - t77694 + t77695 - t77696 + t77697 + t77700 - F::new(0.19957069503106347607e-1) * t1540 * t3230 + t77703 + t77704 + t77705;
    t77706
}
