//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1170/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1170<F: Float>(t4162: F, t8342: F, t8344: F, t23083: F, t32837: F, t23062: F, t32834: F, t1509: F, t23109: F, t23110: F, t232: F, t59: F, t1516: F, t30720: F, t30709: F, t4261: F, t8343: F) -> (F, F, F, F, F, F, F) {
    let t118576 = t4162 * t8342 * t8344;
    let t118578 = t23083 * t32837;
    let t118580 = t23062 * t32834;
    let t118586 = t23109 * t23110 * t59 * t1509 * t232;
    let t118588 = t30720 * t1516;
    let t118590 = t30709 * t1516;
    let t118592 = t8343 * t4261;
    (t118576, t118578, t118580, t118586, t118588, t118590, t118592)
}
