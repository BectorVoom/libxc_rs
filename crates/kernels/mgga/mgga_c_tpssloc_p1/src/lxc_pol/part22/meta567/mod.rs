//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2073;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta567<F: Float>(t13783: F, t984: F, t10277: F, t343: F, t42308: F, t974: F, t2978: F, t698: F, t2402: F, t976: F, t973: F, t979: F) -> (F, F, F, F, F, F) {
        let (t42837, t42841, t42861, t42875, t42891, t42893) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2073::<F>(t13783, t984, t10277, t343, t42308, t974, t2978, t698, t2402, t976, t973, t979);
    (t42837, t42841, t42861, t42875, t42891, t42893)
}
