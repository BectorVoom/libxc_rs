//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta602 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1991;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1992;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta602<F: Float>(t81735: F, t1891: F, t22816: F, t23104: F, t80967: F, t6612: F, t812: F, t836: F, t2690: F, t6619: F, t849: F, t23132: F, t2617: F, t131: F, t23121: F, t9537: F, t236: F, t81613: F, t22822: F, t281: F, t6589: F, t23124: F, t23076: F, t6597: F, t23047: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t81736, t81743, t81749, t81763, t81764, t81769) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1991::<F>(t81735, t1891, t22816, t23104, t80967, t6612, t812, t836, t2690, t6619, t849, t23132, t2617);
        let (t81782, t81783, t81788, t81789, t81792, t81803) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1992::<F>(t131, t23121, t9537, t236, t81613, t22822, t281, t6589, t23124, t23076, t6597, t23047, t2617);
    (t81736, t81743, t81749, t81763, t81764, t81769, t81782, t81783, t81788, t81789, t81792, t81803)
}
