//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 816/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk816<F: Float>(t30690: F, t6547: F, t23030: F, t30681: F, t30689: F, t6562: F, t794: F, t22690: F, t23171: F, t30676: F, t112976: F, t1888: F, t232: F, t6646: F, t82034: F, t6624: F, t828: F) -> (F, F, F, F, F, F, F) {
    let t112991 = t6547 * t30690;
    let t112992 = 0.76763589786250567036e-1 * t112991;
    let t112995 = 0.52089578783527170489e-1 * t23030 * t30681;
    let t112997 = t6562 * t794 * t30689;
    let t112998 = 0.16449340668482264365e-1 * t112997;
    let t113005 = 0.16449340668482264365e-1 * t23171 * t22690 * t30676;
    let t113009 = 0.16449340668482264365e-1 * t1888 * t6646 * t112976 * t232;
    let t113023 = 0.16449340668482264365e-1 * t1888 * t6646 * t82034 * t232;
    let t113032 = 0.3289868133696452873e-1 * t1888 * t6646 * t6624 * t828 * t232;
    (t112992, t112995, t112998, t113005, t113009, t113023, t113032)
}
