//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta614 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta614<F: Float>(t39706: F, t39749: F, t39803: F, t39840: F, t17: F, t521: F, t2225: F, t3826: F, t12129: F, t592: F, t2223: F, t11985: F, t25: F, t514: F) -> (F, F, F, F, F, F) {
        let (t39842, t39844, t39845, t39851, t39857, t39861) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2389::<F>(t39706, t39749, t39803, t39840, t17, t521, t2225, t3826, t12129, t592, t2223, t11985, t25, t514);
    (t39842, t39844, t39845, t39851, t39857, t39861)
}
