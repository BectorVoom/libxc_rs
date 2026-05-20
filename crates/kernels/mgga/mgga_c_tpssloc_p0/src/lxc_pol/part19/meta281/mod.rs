//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1046;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta281<F: Float>(t12167: F, t550: F, t1380: F, t1372: F, t3787: F, t3793: F, t1351: F, t3791: F, t3856: F, t3901: F, t215: F, t535: F, t9569: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t12168, t12169, t12171, t12172, t12177, t12178, t12179, t12181, t12188) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1046::<F>(t12167, t550, t1380, t1372, t3787, t3793, t1351, t3791, t3856, t3901, t215, t535, t9569);
    (t12168, t12169, t12171, t12172, t12177, t12178, t12179, t12181, t12188)
}
