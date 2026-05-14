//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 845/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk845<F: Float>(t77561: F, t1971: F, t2227: F, t515: F, t615: F, t7230: F, t14473: F, t4985: F, t71447: F, t75252: F, t75266: F, t75269: F, t77540: F, t77542: F, t77545: F, t77550: F, t77553: F, t77556: F, t77557: F, t77558: F, t77559: F, t77560: F) -> (F,) {
    let t77562 = 0.53205749866622299248e-5 * t77561;
    let t77566 = t7230 * t1971 * t515 * t2227 * t615;
    let t77567 = 0.53205749866622299248e-5 * t77566;
    let t77568 = t77540 + 0.13469175824740901073e-6 * t75252 + t77542 + 0.59871208509319042821e-1 * t4985 * t14473 + t77545 + 0.58171619854173713846e-5 * t75266 - 0.58171619854173713846e-5 * t75269 + t77550 - t77553 - t77556 - t77557 - t77558 - t77559 + t77560 + t71447 + t77562 + t77567;
    (t77568,)
}
