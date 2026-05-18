//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 968/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk968<F: Float>(t30714: F, t4240: F, t4250: F, t4191: F, t1484: F, t865: F, t23788: F, t4255: F, t25365: F, t25927: F, t25374: F, t89953: F) -> (F, F, F, F, F, F, F) {
    let t118608 = t30714 * t4240;
    let t118610 = t30714 * t4250;
    let t118612 = t30714 * t4191;
    let t118833 = t1484 * t865;
    let t119691 = t23788 * t4255;
    let t119713 = t25927 * t25365;
    let t119755 = t89953 * t25374;
    (t118608, t118610, t118612, t118833, t119691, t119713, t119755)
}
