//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta690 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2505;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta690<F: Float>(t12886: F, t706: F, t157: F, t41284: F, t12923: F, t12939: F, t2244: F, t2250: F, t4194: F, t46528: F, t816: F, t4159: F, t9541: F) -> (F, F, F, F, F, F) {
        let (t47172, t47176, t47180, t47185, t47220, t47230) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2505::<F>(t12886, t706, t157, t41284, t12923, t12939, t2244, t2250, t4194, t46528, t816, t4159, t9541);
    (t47172, t47176, t47180, t47185, t47220, t47230)
}
