//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 710/939 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk710<F: Float>(t25119: F, t25120: F, t4234: F, t815: F, t6605: F, t23083: F, t7500: F, t4159: F, t6581: F, t1509: F, t236: F, t23110: F, t232: F, t23109: F, t1898: F, t4162: F) -> (F, F, F, F, F, F) {
    let t25121 = t25119 * t25120;
    let t25123 = t815 * t4234;
    let t25124 = t6605 * t25123;
    let t25126 = t23083 * t7500;
    let t25128 = t6581 * t4159;
    let t25130 = t236 * t1509;
    let t25132 = t23110 * t25130 * t232;
    let t25133 = t23109 * t25132;
    let t25135 = t4162 * t1898;
    (t25121, t25124, t25126, t25128, t25133, t25135)
}
