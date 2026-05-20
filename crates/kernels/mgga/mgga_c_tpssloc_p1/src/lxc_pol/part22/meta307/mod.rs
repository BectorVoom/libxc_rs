//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta307 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1479;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta307<F: Float>(t14722: F, t14704: F, t11147: F, t1409: F, t11153: F, t3242: F, t3966: F, t3247: F, t1667: F, t2403: F) -> (F, F, F, F, F, F, F) {
        let (t14723, t14724, t14725, t14730, t14735, t14748, t14766) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1479::<F>(t14722, t14704, t11147, t1409, t11153, t3242, t3966, t3247, t1667, t2403);
    (t14723, t14724, t14725, t14730, t14735, t14748, t14766)
}
