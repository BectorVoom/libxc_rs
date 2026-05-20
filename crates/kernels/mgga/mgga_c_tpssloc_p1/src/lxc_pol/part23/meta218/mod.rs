//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta218 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk865;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta218<F: Float>(t1561: F, t2860: F, t1573: F, t2929: F, t1603: F, t3030: F, t3032: F, t3129: F, t3038: F, t3199: F, t3185: F, t1654: F, t2394: F) -> (F, F, F, F, F, F, F) {
        let (t14276, t14337, t14508, t14511, t14608, t14618, t14702) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk865::<F>(t1561, t2860, t1573, t2929, t1603, t3030, t3032, t3129, t3038, t3199, t3185, t1654, t2394);
    (t14276, t14337, t14508, t14511, t14608, t14618, t14702)
}
