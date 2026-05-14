//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 775/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk775<F: Float>(t232: F, t2646: F, t4180: F, t30714: F, t235: F, t835: F, t226: F, t8344: F, t8343: F, t849: F, t8301: F, t9231: F, t645: F, t8307: F, t8513: F, t31: F) -> (F, F, F, F, F, F, F, F, F) {
    let t30716 = t4180 * t2646 * t232;
    let t30717 = t30714 * t30716;
    let t30719 = t235 * t835;
    let t30720 = t226 * t30719;
    let t30721 = t30720 * t8344;
    let t30723 = t8343 * t849;
    let t31000 = t9231 * t8301;
    let t31005 = t8307 * t645;
    let t31006 = t8513 * t31005;
    let t31011 = t8307 * t31;
    (t30716, t30717, t30719, t30720, t30721, t30723, t31000, t31006, t31011)
}
