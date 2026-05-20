//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta581 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1867;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1868;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta581<F: Float>(t23097: F, t2628: F, t2632: F, t47012: F, t23033: F, t25155: F, t6546: F, t13191: F, t221: F, t25154: F, t13196: F, t13171: F, t6605: F, t815: F, t58300: F, t25112: F, t81835: F, t232: F, t47262: F, t23083: F, t25116: F, t1510: F, t2553: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t87458, t87463, t87466, t87469, t87472) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1867::<F>(t23097, t2628, t2632, t47012, t23033, t25155, t6546, t13191, t221, t25154, t13196, t13171, t6605, t815);
        let (t87475, t87477, t87481, t87485, t87487, t87491) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1868::<F>(t58300, t6605, t815, t25112, t81835, t232, t47262, t23097, t47012, t23083, t25116, t1510, t2553);
    (t87458, t87463, t87466, t87469, t87472, t87475, t87477, t87481, t87485, t87487, t87491)
}
