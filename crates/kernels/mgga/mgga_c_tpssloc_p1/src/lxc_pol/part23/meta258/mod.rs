//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta258 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk919;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta258<F: Float>(t562: F, t6414: F, t172: F, t6320: F, t763: F, t1819: F, t68: F, t1995: F, t6330: F, t1824: F, t1834: F, t6387: F) -> (F, F, F, F, F, F, F) {
        let (t19660, t19681, t19682, t19708, t19715, t19739, t19743) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk919::<F>(t562, t6414, t172, t6320, t763, t1819, t68, t1995, t6330, t1824, t1834, t6387);
    (t19660, t19681, t19682, t19708, t19715, t19739, t19743)
}
