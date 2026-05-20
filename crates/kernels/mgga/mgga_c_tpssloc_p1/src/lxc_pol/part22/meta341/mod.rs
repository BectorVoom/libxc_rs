//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta341 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1541;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta341<F: Float>(t16689: F, t708: F, t9871: F, t13115: F, t157: F, t4196: F, t9880: F, t13107: F, t13105: F, t16688: F, t9793: F, t9797: F, t9820: F, t9824: F, t9876: F, t9884: F, t9887: F, t9890: F) -> (F, F, F, F, F, F, F) {
        let (t16691, t16692, t16693, t16695, t16696, t16697, t16698) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1541::<F>(t16689, t708, t9871, t13115, t157, t4196, t9880, t13107, t13105, t16688, t9793, t9797, t9820, t9824, t9876, t9884, t9887, t9890);
    (t16691, t16692, t16693, t16695, t16696, t16697, t16698)
}
