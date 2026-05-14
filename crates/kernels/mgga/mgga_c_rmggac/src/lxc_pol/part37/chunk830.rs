//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 830/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk830<F: Float>(t77671: F, t75598: F, t14424: F, t4985: F, t14427: F, t5928: F, t14469: F, t2868: F, t75615: F, t75620: F, t75626: F, t75629: F, t75632: F, t75635: F, t69907: F, t14584: F, t623: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t77672 = 0.18183107769496894486e-1 * t77671;
    let t77677 = 0.15961724959986689775e-4 * t75598;
    let t77679 = 0.11974241701863808564e0 * t4985 * t14424;
    let t77681 = 0.11974241701863808564e0 * t5928 * t14427;
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
    (t77672, t77677, t77679, t77681, t77686, t77690, t77691, t77693, t77694, t77695, t77696, t77697, t77698)
}
