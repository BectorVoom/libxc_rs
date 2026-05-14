//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 825/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk825<F: Float>(t1356: F, t74292: F, t8041: F, t75271: F, t75273: F, t75277: F, t69689: F, t15626: F, t34847: F, t1971: F, t2227: F, t515: F, t615: F, t7230: F, t1525: F, t698: F) -> (F, F, F, F, F, F, F, F) {
    let t77556 = 0.11974241701863808564e0 * t1356 * t8041 * t74292;
    let t77557 = 0.20455996240684006296e-1 * t75271;
    let t77558 = 0.20455996240684006296e-1 * t75273;
    let t77559 = 0.20455996240684006296e-1 * t75277;
    let t77560 = 0.18183107769496894487e-1 * t69689;
    let t77561 = t34847 * t15626;
    let t77562 = 0.53205749866622299248e-5 * t77561;
    let t77566 = t7230 * t1971 * t515 * t2227 * t615;
    let t77567 = 0.53205749866622299248e-5 * t77566;
    let t77572 = t7230 * t1971 * t515 * t698 * t1525;
    (t77556, t77557, t77558, t77559, t77560, t77562, t77567, t77572)
}
