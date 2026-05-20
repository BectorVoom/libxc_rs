//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta344 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1738;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta344<F: Float>(t9864: F, t9866: F, t3966: F, t751: F, t707: F, t2379: F, t262: F, t157: F, t9897: F) -> (F, F, F, F, F, F) {
        let (t12927, t12928, t12932, t12934, t12935, t12939) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1738::<F>(t9864, t9866, t3966, t751, t707, t2379, t262, t157, t9897);
    (t12927, t12928, t12932, t12934, t12935, t12939)
}
