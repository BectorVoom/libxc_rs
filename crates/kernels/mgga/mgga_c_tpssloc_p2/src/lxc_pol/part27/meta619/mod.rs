//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta619 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2097;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta619<F: Float>(t6695: F, t82632: F, t23317: F, t23384: F, t225: F, t23572: F, t23587: F, t6698: F, t3166: F, t6688: F, t23399: F, t6692: F, t82573: F) -> (F, F, F, F, F, F, F) {
        let (t83368, t83398, t83408, t83420, t83424, t83435, t83441) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2097::<F>(t6695, t82632, t23317, t23384, t225, t23572, t23587, t6698, t3166, t6688, t23399, t6692, t82573);
    (t83368, t83398, t83408, t83420, t83424, t83435, t83441)
}
