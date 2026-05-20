//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta665 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta665<F: Float>(t16944: F, t25891: F, t25927: F, t98111: F, t1649: F, t4119: F, t23788: F, t67123: F, t1081: F, t5660: F, t5544: F, t16662: F, t28: F) -> (F, F, F, F, F, F, F) {
        let (t100708, t100713, t100718, t100731, t100734, t100743, t100747) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1954::<F>(t16944, t25891, t25927, t98111, t1649, t4119, t23788, t67123, t1081, t5660, t5544, t16662, t28);
    (t100708, t100713, t100718, t100731, t100734, t100743, t100747)
}
