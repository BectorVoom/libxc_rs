//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1640;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta341<F: Float>(t522: F, t9212: F, t9214: F, t3824: F, t592: F, t11976: F, t11978: F, t11980: F, t11982: F, t11984: F, t9457: F, t9476: F, t9484: F, t9780: F) -> (F, F, F, F, F) {
        let (t12044, t12045, t12046, t12048, t12049) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1640::<F>(t522, t9212, t9214, t3824, t592, t11976, t11978, t11980, t11982, t11984, t9457, t9476, t9484, t9780);
    (t12044, t12045, t12046, t12048, t12049)
}
