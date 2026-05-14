//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 856/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk856<F: Float>(t77754: F, t15624: F, t1971: F, t2144: F, t333: F, t7230: F, t352: F, t875: F, t118: F, t1986: F, t615: F, t699: F, t7717: F, t75675: F, t75681: F, t75685: F) -> (F, F, F, F, F, F, F) {
    let t77755 = 0.15961724959986689774e-4 * t77754;
    let t77759 = t7230 * t1971 * t2144 * t15624 * t333;
    let t77760 = 0.15961724959986689774e-4 * t77759;
    let t77764 = t7230 * t1971 * t875 * t15624 * t352;
    let t77765 = 0.1064114997332445985e-4 * t77764;
    let t77768 = t1986 * t118 * t699 * t615;
    let t77769 = t7717 * t77768;
    let t77770 = 0.53205749866622299248e-5 * t77769;
    let t77772 = 0.79828278012425390427e-1 * t75675;
    let t77773 = 0.1276937996798935182e-4 * t75681;
    let t77774 = 0.15961724959986689775e-4 * t75685;
    (t77755, t77760, t77765, t77770, t77772, t77773, t77774)
}
