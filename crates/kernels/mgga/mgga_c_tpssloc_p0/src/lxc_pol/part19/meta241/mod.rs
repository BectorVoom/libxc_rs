//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta241 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk971;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk972;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta241<F: Float>(t11159: F, t3297: F, t136: F, t1113: F, t11168: F, t407: F, t1102: F, t3271: F, t11135: F, t11137: F, t11139: F, t11141: F, t11143: F, t11150: F, t11156: F, t11161: F, t11165: F, t11170: F, t11174: F, t1100: F, t1107: F, t410: F, t417: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t11229, t11230, t11232, t11233, t11243, t11244, t11245, t11258) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk971::<F>(t11159, t3297, t136, t1113, t11168, t407, t1102, t3271, t11135, t11137, t11139, t11141, t11143, t11150, t11156, t11161, t11165, t11170, t11174);
        let (t11259, t11261, t11265, t11266, t11268) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk972::<F>(t1100, t11258, t1107, t410, t417, t11244, t11137, t11139, t11141, t11143, t11150, t11156, t11165, t11174, t11230, t11233, t11245);
    (t11229, t11230, t11232, t11233, t11243, t11245, t11258, t11259, t11261, t11265, t11266, t11268)
}
