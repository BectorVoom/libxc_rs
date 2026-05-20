//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1818;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1819;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta580<F: Float>(t22716: F, t7701: F, t1834: F, t212: F, t22642: F, t6890: F, t81267: F, t26215: F, t81228: F, t81326: F, t6897: F, t6907: F, t90544: F, t81284: F, t26203: F, t6883: F, t7700: F, t80645: F, t214: F, t5318: F, t81311: F, t26378: F, t6914: F, t1372: F, t1799: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90659, t90663, t90670, t90686, t90701) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1818::<F>(t22716, t7701, t1834, t212, t22642, t6890, t81267, t26215, t81228, t81326, t6897, t6907, t90544);
        let (t90706, t90707, t90723, t90739, t90743, t90749, t90754) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1819::<F>(t81284, t26203, t6883, t6897, t7700, t80645, t214, t5318, t81311, t26378, t6914, t1372, t1799);
    (t90659, t90663, t90670, t90686, t90701, t90706, t90707, t90723, t90739, t90743, t90749, t90754)
}
