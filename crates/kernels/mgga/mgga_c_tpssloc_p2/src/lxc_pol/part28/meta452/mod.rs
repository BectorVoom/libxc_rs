//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1647;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1648;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta452<F: Float>(t24115: F, t24137: F, t1378: F, t1323: F, t7191: F, t1385: F, t7213: F, t3887: F, t22923: F, t22925: F, t2085: F, t3752: F, t1375: F, t22664: F, t22668: F, t22676: F, t22688: F, t22907: F, t22909: F, t22918: F, t22921: F, t22928: F, t22931: F, t22936: F, t22940: F, t568: F) -> (F, F, F, F, F, F, F, F) {
        let (t24138, t24139, t24141, t24147, t24156, t24157, t24162) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1647::<F>(t24115, t24137, t1378, t1323, t7191, t1385, t7213, t3887, t22923, t22925, t2085, t3752);
        let t24164 = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1648::<F>(t1375, t22664, t22668, t22676, t22688, t22907, t22909, t22918, t22921, t22928, t22931, t22936, t22940, t24139, t24141, t24147, t24156, t24157, t24162, t568);
    (t24138, t24139, t24141, t24147, t24156, t24157, t24162, t24164)
}
