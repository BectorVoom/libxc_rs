//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 796/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk796<F: Float>(t33518: F, t33552: F, t113: F, t7756: F, t8607: F, t1442: F, t8595: F, t1976: F, t32674: F, t32676: F, t32679: F, t33360: F, t33361: F, t33364: F, t33365: F, t33367: F, t7787: F, t7941: F, t8329: F, t8450: F) -> (F, F) {
    let t33553 = t33518 + t33552;
    let t33554 = t113 * t33553;
    let t33555 = t8607 * t7756;
    let t33556 = t1442 * t8595;
    let t33558 = -t1976 * t7787 + t7941 * t8450 - t32674 - t32676 - t32679 - t33360 - t33361 + t33364 + t33365 - t33367 - t33554 - t33555 - t33556 - t8329;
    (t33553, t33558)
}
