//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 655/916 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk655<F: Float>(t70078: F, t70082: F, t14494: F, t874: F, t14563: F, t2160: F, t638: F, t14559: F, t70188: F, t70237: F, t70271: F, t14530: F, t290: F, t14580: F, t899: F, t70316: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t71671 = 0.39032073591371545778e-3 * t70078;
    let t71672 = 0.30487649791575028312e-3 * t70082;
    let t71704 = t874 * t14494;
    let t71717 = t638 * t2160 * t14563;
    let t71720 = t638 * t2160 * t14559;
    let t71727 = 0.46328831667894726564e-5 * t70188;
    let t71744 = 0.60975299583150056624e-3 * t70237;
    let t71755 = 0.6505345598561924296e-5 * t70271;
    let t71760 = t290 * t14530;
    let t71772 = t899 * t14580;
    let t71775 = 0.6505345598561924296e-5 * t70316;
    (t71671, t71672, t71704, t71717, t71720, t71727, t71744, t71755, t71760, t71772, t71775)
}
