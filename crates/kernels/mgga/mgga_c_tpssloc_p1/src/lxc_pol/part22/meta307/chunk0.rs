//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1479/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1479<F: Float>(t14722: F, t14704: F, t11147: F, t1409: F, t11153: F, t3242: F, t3966: F, t3247: F, t1667: F, t2403: F) -> (F, F, F, F, F, F, F) {
    let t14723 = F::new(4.0) / F::new(9.0) * t14722;
    let t14724 = F::new(2.0) / F::new(9.0) * t14704;
    let t14725 = t11147 * t1409;
    let t14730 = t11153 * t1409;
    let t14735 = t3242 * t3966;
    let t14748 = t3247 * t3966;
    let t14766 = t2403 * t1667;
    (t14723, t14724, t14725, t14730, t14735, t14748, t14766)
}
