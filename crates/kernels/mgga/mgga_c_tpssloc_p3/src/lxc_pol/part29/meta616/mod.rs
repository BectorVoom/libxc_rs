//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta616 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2057;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta616<F: Float>(t27495: F, t85821: F, t15702: F, t7329: F, t1011: F, t3493: F, t225: F, t24698: F, t1193: F, t24811: F, t24817: F, t24823: F) -> (F, F, F, F, F, F, F) {
        let (t85822, t85824, t85827, t85832, t85853, t85854, t85883) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2057::<F>(t27495, t85821, t15702, t7329, t1011, t3493, t225, t24698, t1193, t24811, t24817, t24823);
    (t85822, t85824, t85827, t85832, t85853, t85854, t85883)
}
