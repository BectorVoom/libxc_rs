//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta644 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1914;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1915;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta644<F: Float>(t22986: F, t25054: F, t86873: F, t6552: F, t6555: F, t98133: F, t1880: F, t25216: F, t25224: F, t25038: F, t25040: F, t28267: F, t81651: F, t82074: F, t1888: F, t23270: F, t25044: F, t4300: F, t5527: F, t857: F, t865: F, t23035: F, t23237: F, t28298: F, t23204: F, t81640: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t98196, t98199, t98202, t98205, t98213) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1914::<F>(t22986, t25054, t86873, t6552, t6555, t98133, t1880, t25216, t25224, t25038, t25040, t28267, t81651, t82074);
        let (t98222, t98227, t98234, t98237) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1915::<F>(t1888, t23270, t25044, t4300, t5527, t857, t25038, t865, t23035, t23237, t28298, t23204, t81640);
    (t98196, t98199, t98202, t98205, t98213, t98222, t98227, t98234, t98237)
}
