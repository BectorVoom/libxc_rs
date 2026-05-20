//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 924/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk924<F: Float>(t1509: F, t23109: F, t23110: F, t232: F, t59: F, t1516: F, t30720: F, t112802: F, t32844: F, t1499: F, t30719: F, t8344: F) -> (F, F, F, F) {
    let t118586 = t23109 * t23110 * t59 * t1509 * t232;
    let t118588 = t30720 * t1516;
    let t118596 = t112802 * t32844;
    let t118602 = t1499 * t30719 * t8344;
    (t118586, t118588, t118596, t118602)
}
