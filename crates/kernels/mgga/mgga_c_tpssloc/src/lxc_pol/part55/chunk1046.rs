//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1046/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1046<F: Float>(t32649: F, t576: F, t112: F, t32629: F, t111: F, t8919: F, t193: F, t8421: F, t25374: F, t86716: F, t200: F, t8365: F, t25: F, t25353: F, t606: F, t7540: F) -> (F, F, F, F, F, F, F, F) {
    let t118347 = t576 * t32649;
    let t118354 = t32629 * t112;
    let t118365 = t8919 * t111;
    let t118376 = t193 * t8421;
    let t118377 = t86716 * t25374;
    let t118381 = t193 * t200 * t8365;
    let t118387 = t25 * t25353;
    let t118393 = t606 * t7540;
    (t118347, t118354, t118365, t118376, t118377, t118381, t118387, t118393)
}
