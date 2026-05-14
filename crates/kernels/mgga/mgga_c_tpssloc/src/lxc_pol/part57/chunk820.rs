//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 820/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk820<F: Float>(t7537: F, t857: F, t32815: F, t81591: F, t30713: F, t4166: F, t1484: F, t22690: F, t23122: F, t6619: F, t23083: F, t32837: F, t23062: F, t32834: F, t1509: F, t23109: F, t23110: F, t232: F, t59: F) -> (F, F, F, F, F, F, F) {
    let t118472 = t857 * t7537;
    let t118480 = t81591 * t32815;
    let t118532 = t4166 * t30713;
    let t118573 = t23122 * t22690 * t6619 * t1484;
    let t118578 = t23083 * t32837;
    let t118580 = t23062 * t32834;
    let t118586 = t23109 * t23110 * t59 * t1509 * t232;
    (t118472, t118480, t118532, t118573, t118578, t118580, t118586)
}
