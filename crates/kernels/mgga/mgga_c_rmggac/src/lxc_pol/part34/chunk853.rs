//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 853/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk853<F: Float>(t14469: F, t2868: F, t75615: F, t75620: F, t75626: F, t75629: F, t75632: F, t75635: F, t69907: F, t14584: F, t623: F, t2141: F, t75638: F, t75640: F, t75644: F, t14501: F, t1540: F, t3230: F, t75607: F, t75611: F, t75623: F) -> (F,) {
    let t77685 = t2868 * t14469;
    let t77686 = 0.39914139006212695213e-1 * t77685;
    let t77690 = 0.23268647941669485538e-4 * t75615;
    let t77691 = 0.3941843870902807617e-5 * t75620;
    let t77693 = 0.5255791827870410156e-5 * t75626;
    let t77694 = 0.1276937996798935182e-4 * t75629;
    let t77695 = 0.2553875993597870364e-4 * t75632;
    let t77696 = 0.3830813990396805546e-4 * t75635;
    let t77697 = 0.18183107769496894487e-1 * t69907;
    let t77698 = t623 * t14584;
    let t77699 = t77698 * t2141;
    let t77700 = 0.13637330827122670864e-1 * t77699;
    let t77703 = 0.14967802127329760705e-1 * t75638;
    let t77704 = 0.10227998120342003148e-1 * t75640;
    let t77705 = 0.10227998120342003148e-1 * t75644;
    let t77706 = -t77686 - 0.59871208509319042821e-1 * t2868 * t14501 - t75607 - 0.17451485956252114154e-4 * t75611 - t77690 - t77691 - 0.17519306092901367187e-5 * t75623 + t77693 - t77694 + t77695 - t77696 + t77697 + t77700 - 0.19957069503106347607e-1 * t1540 * t3230 + t77703 + t77704 + t77705;
    (t77706,)
}
