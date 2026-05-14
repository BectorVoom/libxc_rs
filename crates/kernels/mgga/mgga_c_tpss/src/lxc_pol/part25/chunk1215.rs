//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1215/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1215<F: Float>(t5059: F, t821: F, t20047: F, t69803: F, t750: F, t14426: F, t33: F, t1497: F, t3610: F, t19809: F, t64975: F, t18246: F, t52639: F, t69810: F, t69847: F, t14245: F, t20011: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t70850 = t5059 * t821;
    let t70854 = t20047 * t69803;
    let t70857 = t5059 * t750;
    let t70861 = t33 * t14426;
    let t70868 = t1497 * t3610;
    let t70872 = t64975 * t19809;
    let t70887 = t18246 * t52639;
    let t70890 = t18246 * t69810;
    let t70893 = t20047 * t69847;
    let t70906 = t20011 * t14245;
    (t70850, t70854, t70857, t70861, t70868, t70872, t70887, t70890, t70893, t70906)
}
