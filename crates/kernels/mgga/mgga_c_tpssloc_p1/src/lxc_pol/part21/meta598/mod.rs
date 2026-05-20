//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta598 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2349;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2350;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta598<F: Float>(t20: F, t60: F, t1799: F, t3701: F, t9108: F, t94: F, t102: F, t9174: F, t2: F, t591: F, t21: F, t9: F, t14: F, t2230: F, t22811: F, t19: F, t2233: F, t2239: F, t601: F, t9238: F, t85: F, t24: F, t10276: F, t73: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t32253, t33159, t35577, t35761, t39031, t39033) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2349::<F>(t20, t60, t1799, t3701, t9108, t94, t102, t9174, t2, t591, t21, t9);
        let (t39037, t39043, t39049, t39054, t39063, t39096) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2350::<F>(t14, t2230, t22811, t19, t2233, t2239, t601, t9238, t85, t24, t10276, t73);
    (t32253, t33159, t35577, t35761, t39031, t39033, t39037, t39043, t39049, t39054, t39063, t39096)
}
