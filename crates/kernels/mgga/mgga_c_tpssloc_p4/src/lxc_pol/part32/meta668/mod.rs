//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta668 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta668<F: Float>(t24574: F, t27484: F, t24826: F, t27540: F, t210: F, t24848: F, t27505: F, t27466: F, t27455: F, t27474: F, t27492: F, t85853: F) -> (F, F, F, F, F, F, F) {
        let (t95048, t95069, t95092, t95098, t95114, t95125, t95134) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2100::<F>(t24574, t27484, t24826, t27540, t210, t24848, t27505, t27466, t27455, t27474, t27492, t85853);
    (t95048, t95069, t95092, t95098, t95114, t95125, t95134)
}
