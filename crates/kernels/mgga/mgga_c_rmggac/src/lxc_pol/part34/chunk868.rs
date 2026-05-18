//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 868/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk868<F: Float>(t14125: F, t21708: F, t8503: F, t21709: F, t8507: F, t15384: F, t34847: F, t1971: F, t2123: F, t515: F, t615: F, t7230: F) -> (F, F, F, F) {
    let t75533 = t21708 * t14125 * t8503;
    let t75536 = t21708 * t21709 * t8507;
    let t75539 = F::new(0.1064114997332445985e-4) * t34847 * t15384;
    let t75545 = F::new(0.1064114997332445985e-4) * t7230 * t1971 * t515 * t2123 * t615;
    (t75533, t75536, t75539, t75545)
}
