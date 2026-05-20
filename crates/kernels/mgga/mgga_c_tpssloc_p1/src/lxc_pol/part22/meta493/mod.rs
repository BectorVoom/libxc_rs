//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta493 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1919;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1920;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta493<F: Float>(t17800: F, t4514: F, t17794: F, t4531: F, t10339: F, t13896: F, t17764: F, t17770: F, t17827: F, t17850: F, t21410: F, t21413: F, t21416: F, t2986: F, t973: F, t17817: F, t17804: F, t10295: F, t13642: F, t17286: F, t17288: F, t17290: F, t21120: F, t21132: F, t21136: F, t21140: F, t21161: F, t21168: F) -> (F, F, F, F, F, F) {
        let (t21419, t21422, t21429) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1919::<F>(t17800, t4514, t17794, t4531, t10339, t13896, t17764, t17770, t17827, t17850, t21410, t21413, t21416, t2986, t973);
        let (t21430, t21433, t21444) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1920::<F>(t17817, t4531, t17804, t4514, t10295, t13642, t17286, t17288, t17290, t21120, t21132, t21136, t21140, t21161, t21168);
    (t21419, t21422, t21429, t21430, t21433, t21444)
}
