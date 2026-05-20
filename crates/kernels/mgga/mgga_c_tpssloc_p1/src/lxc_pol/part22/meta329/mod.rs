//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta329 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1516;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta329<F: Float>(t1307: F, t16095: F, t686: F, t16094: F, t12214: F, t131: F, t205: F, t3726: F, t5206: F, t12199: F, t5202: F, t118: F, t5187: F, t794: F) -> (F, F, F, F, F, F, F) {
        let (t16097, t16099, t16100, t16101, t16106, t16108, t16111) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1516::<F>(t1307, t16095, t686, t16094, t12214, t131, t205, t3726, t5206, t12199, t5202, t118, t5187, t794);
    (t16097, t16099, t16100, t16101, t16106, t16108, t16111)
}
