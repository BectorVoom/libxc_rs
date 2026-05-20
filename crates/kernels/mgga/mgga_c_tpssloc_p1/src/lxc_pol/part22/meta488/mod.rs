//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta488 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1908;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1909;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta488<F: Float>(t21114: F, t932: F, t1557: F, t17195: F, t4354: F, t5727: F, t13520: F, t5730: F, t21252: F, t2844: F, t10661: F, t10675: F, t10676: F, t21120: F, t21124: F, t21128: F, t21132: F, t21136: F, t21140: F, t21142: F, t21144: F, t21147: F, t21150: F, t21153: F, t21156: F, t13598: F, t13642: F, t17149: F, t17165: F, t17175: F, t17286: F, t17288: F, t17290: F, t21161: F, t21168: F, t21181: F, t21183: F, t21186: F, t21188: F) -> (F, F, F, F, F, F, F, F) {
        let (t21259, t21263, t21265, t21267, t21268, t21270, t21283) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1908::<F>(t21114, t932, t1557, t17195, t4354, t5727, t13520, t5730, t21252, t2844, t10661, t10675, t10676, t21120, t21124, t21128, t21132, t21136, t21140, t21142, t21144, t21147, t21150, t21153, t21156);
        let t21298 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1909::<F>(t13598, t13642, t17149, t17165, t17175, t17286, t17288, t17290, t21161, t21168, t21181, t21183, t21186, t21188);
    (t21259, t21263, t21265, t21267, t21268, t21270, t21283, t21298)
}
