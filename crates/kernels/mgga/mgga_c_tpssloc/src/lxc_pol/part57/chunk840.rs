//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 840/919 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk840<F: Float>(t33153: F, t33151: F, t33106: F, t7440: F, t8513: F, t119942: F, t1433: F, t119878: F, t1409: F, t1410: F, t8308: F, t2240: F, t32: F, t5392: F, t27948: F, t8307: F) -> (F, F, F, F, F, F, F, F, F) {
    let t126035 = 4.0 * t33153;
    let t126036 = 4.0 * t33151;
    let t126046 = t8513 * t33106 * t7440;
    let t126062 = t8513 * t119942 * t1433;
    let t126065 = t119878 * t1409;
    let t126070 = t8308 * t1410 * t7440;
    let t126073 = t1410 * t1433;
    let t126091 = t2240 * t32 * t5392;
    let t126100 = t8513 * t8307 * t27948;
    (t126035, t126036, t126046, t126062, t126065, t126070, t126073, t126091, t126100)
}
