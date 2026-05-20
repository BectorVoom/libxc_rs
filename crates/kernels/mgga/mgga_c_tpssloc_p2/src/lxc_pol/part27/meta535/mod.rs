//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta535 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1953;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1954;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta535<F: Float>(t3953: F, t608: F, t1437: F, t641: F, t72: F, t4021: F, t79: F, t1410: F, t2235: F, t3961: F, t605: F, t3967: F, t33: F, t7440: F, t2240: F, t1433: F, t645: F, t1865: F, t22523: F, t22554: F, t6490: F, t6492: F, t6495: F, t6506: F, t6510: F, t7432: F, t7435: F, t7442: F, t7446: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t26055, t26062, t26063, t26066, t26067, t26070, t26073, t26076) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1953::<F>(t3953, t608, t1437, t641, t72, t4021, t79, t1410, t2235, t3961, t605, t3967);
        let (t26083, t26084, t26090, t26095) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1954::<F>(t33, t7440, t2240, t1433, t645, t72, t1865, t22523, t22554, t26055, t26063, t26067, t26070, t26073, t26076, t6490, t6492, t6495, t6506, t6510, t7432, t7435, t7442, t7446);
    (t26055, t26062, t26063, t26066, t26067, t26070, t26073, t26076, t26083, t26084, t26090, t26095)
}
