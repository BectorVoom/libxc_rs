//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2019;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2020;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta617<F: Float>(t1016: F, t3034: F, t1081: F, t2752: F, t608: F, t9239: F, t835: F, t531: F, t6995: F, t22573: F, t6875: F, t111: F, t7415: F, t24525: F, t39063: F, t7245: F, t39054: F, t50: F, t9300: F, t11588: F, t2127: F, t221: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t82985, t83555, t83717, t83803, t83859, t83886, t85416) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2019::<F>(t1016, t3034, t1081, t2752, t608, t9239, t835, t531, t6995, t22573, t6875, t111, t7415);
        let (t85480, t85501, t85536, t85539, t85639) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2020::<F>(t24525, t9239, t39063, t7245, t39054, t50, t9300, t11588, t2127, t221);
    (t82985, t83555, t83717, t83803, t83859, t83886, t85416, t85480, t85501, t85536, t85539, t85639)
}
