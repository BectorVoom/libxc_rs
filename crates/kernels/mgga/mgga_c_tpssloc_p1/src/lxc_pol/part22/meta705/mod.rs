//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta705 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2294;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2295;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta705<F: Float>(t18375: F, t3536: F, t11697: F, t18968: F, t3577: F, t11539: F, t1174: F, t18232: F, t18215: F, t11665: F, t18371: F, t15569: F, t15572: F, t1244: F, t3068: F, t478: F, t6163: F, t18386: F, t15608: F, t15740: F, t6183: F, t698: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t66554, t66566, t66571, t66575, t66597, t66599) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2294::<F>(t18375, t3536, t11697, t18968, t3577, t11539, t1174, t18232, t18215, t11665, t18371, t15569, t15572);
        let (t66622, t66646, t66648, t66668) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2295::<F>(t1244, t3068, t478, t6163, t11697, t18386, t3577, t15608, t15740, t1174, t6183, t698);
    (t66554, t66566, t66571, t66575, t66597, t66599, t66622, t66646, t66648, t66668)
}
