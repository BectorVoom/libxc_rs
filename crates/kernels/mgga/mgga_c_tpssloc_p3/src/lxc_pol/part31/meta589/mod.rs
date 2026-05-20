//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1832;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1833;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta589<F: Float>(t1827: F, t80991: F, t22765: F, t5289: F, t22764: F, t5234: F, t1354: F, t26298: F, t80958: F, t22779: F, t26319: F, t1358: F, t26248: F, t3862: F, t7715: F, t22705: F, t22852: F, t236: F, t5286: F, t550: F, t26245: F, t80791: F, t80867: F, t26271: F, t80836: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t91281, t91283, t91285, t91286, t91290, t91300, t91303) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1832::<F>(t1827, t80991, t22765, t5289, t22764, t5234, t1354, t26298, t80958, t22779, t26319, t1358, t26248);
        let (t91305, t91310, t91312, t91314, t91323) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1833::<F>(t3862, t7715, t22705, t22852, t236, t5286, t550, t26245, t80791, t80867, t26271, t80836);
    (t91281, t91283, t91285, t91286, t91290, t91300, t91303, t91305, t91310, t91312, t91314, t91323)
}
