//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 842/943 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk842<F: Float>(t1509: F, t23109: F, t23110: F, t232: F, t59: F, t1516: F, t30720: F, t112802: F, t32844: F, t1499: F, t30719: F, t8344: F, t32814: F, t81651: F, t82074: F, t23168: F, t32789: F) -> (F, F, F, F, F, F) {
    let t118586 = t23109 * t23110 * t59 * t1509 * t232;
    let t118588 = t30720 * t1516;
    let t118596 = t112802 * t32844;
    let t118602 = t1499 * t30719 * t8344;
    let t118632 = t81651 * t82074 * t32814;
    let t118649 = t23168 * t32789;
    (t118586, t118588, t118596, t118602, t118632, t118649)
}
