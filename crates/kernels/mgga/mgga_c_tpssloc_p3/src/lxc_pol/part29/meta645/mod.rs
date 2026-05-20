//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta645 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2128;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2129;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2130;
use chunk3::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta645<F: Float>(t7500: F, t81911: F, t81928: F, t81934: F, t81943: F, t22690: F, t23122: F, t4119: F, t841: F, t25064: F, t81902: F, t23077: F, t6646: F, t23098: F, t7496: F, t6590: F, t25130: F, t81918: F, t81921: F, t81924: F, t81926: F, t81936: F, t87418: F, t87422: F, t87426: F, t87428: F, t87430: F, t23097: F, t2628: F, t2632: F, t47012: F, t23033: F, t25155: F, t6546: F, t13191: F, t221: F, t25154: F, t13196: F, t13171: F, t6605: F, t815: F, t58300: F, t25112: F, t81835: F, t232: F, t47262: F, t23083: F, t25116: F, t1510: F, t2553: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t87432, t87437, t87438, t87440, t87444, t87445, t87447) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2128::<F>(t7500, t81911, t81928, t81934, t81943, t22690, t23122, t4119, t841, t25064, t81902, t23077, t6646);
        let t87455 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2129::<F>(t23098, t7496, t87447, t6590, t6646, t25130, t81918, t81921, t81924, t81926, t81936, t87418, t87422, t87426, t87428, t87430, t87432, t87437, t87438, t87440, t87444, t87445);
        let (t87458, t87464, t87466, t87469, t87472) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2130::<F>(t23097, t2628, t2632, t47012, t23033, t25155, t6546, t13191, t221, t25154, t13196, t13171, t6605, t815);
        let (t87475, t87478, t87481, t87485, t87488, t87491) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2131::<F>(t58300, t6605, t815, t25112, t81835, t232, t47262, t23097, t47012, t23083, t25116, t1510, t2553);
    (t87455, t87458, t87464, t87466, t87469, t87472, t87475, t87478, t87481, t87485, t87488, t87491)
}
