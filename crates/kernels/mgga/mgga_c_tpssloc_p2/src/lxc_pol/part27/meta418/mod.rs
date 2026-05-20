//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1723;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1724;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta418<F: Float>(t22479: F, t510: F, t652: F, t1976: F, t2363: F, t2303: F, t71: F, t1863: F, t33: F, t9228: F, t43: F, t614: F, t2267: F, t38: F, t240: F, t2244: F, t2250: F, t2261: F, t44: F, t607: F, t6500: F, t67: F, t1864: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22480, t22482, t22483, t22489, t22490, t22493, t22502) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1723::<F>(t22479, t510, t652, t1976, t2363, t2303, t71, t1863, t33, t9228, t43, t614);
        let (t22505, t22510, t22511, t22512, t22513) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1724::<F>(t2267, t38, t240, t2244, t2250, t22502, t2261, t44, t607, t6500, t67, t1864);
    (t22480, t22482, t22483, t22489, t22490, t22493, t22502, t22505, t22510, t22511, t22512, t22513)
}
