//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta544 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1767;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1768;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta544<F: Float>(t131: F, t2587: F, t81142: F, t1905: F, t9537: F, t81151: F, t23172: F, t133: F, t1891: F, t6601: F, t80953: F, t22816: F, t23104: F, t80967: F, t6612: F, t812: F, t836: F, t2690: F, t6619: F, t849: F, t23132: F, t2617: F, t23121: F, t236: F, t81613: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81686, t81688, t81715, t81716, t81735, t81742) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1767::<F>(t131, t2587, t81142, t1905, t9537, t81151, t23172, t133, t1891, t6601, t80953, t22816, t23104, t80967);
        let (t81749, t81763, t81764, t81769, t81782, t81783) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1768::<F>(t6612, t812, t836, t2690, t6619, t849, t23132, t2617, t131, t23121, t9537, t236, t81613);
    (t81686, t81688, t81715, t81716, t81735, t81742, t81749, t81763, t81764, t81769, t81782, t81783)
}
