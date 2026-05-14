//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 844/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk844<F: Float>(t75250: F, t75254: F, t75262: F, t2211: F, t41091: F, t739: F, t41006: F, t884: F, t1356: F, t74292: F, t8041: F, t75271: F, t75273: F, t75277: F, t69689: F, t15626: F, t34847: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t77540 = 0.60611291211334054834e-6 * t75250;
    let t77542 = 0.2727466165424534173e-1 * t75254;
    let t77545 = 0.23268647941669485538e-4 * t75262;
    let t77550 = 0.11974241701863808564e0 * t739 * t2211 * t41091;
    let t77553 = 0.11974241701863808564e0 * t884 * t2211 * t41006;
    let t77556 = 0.11974241701863808564e0 * t1356 * t8041 * t74292;
    let t77557 = 0.20455996240684006296e-1 * t75271;
    let t77558 = 0.20455996240684006296e-1 * t75273;
    let t77559 = 0.20455996240684006296e-1 * t75277;
    let t77560 = 0.18183107769496894487e-1 * t69689;
    let t77561 = t34847 * t15626;
    (t77540, t77542, t77545, t77550, t77553, t77556, t77557, t77558, t77559, t77560, t77561)
}
