//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2024;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2025;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta620<F: Float>(t7365: F, t85660: F, t131: F, t467: F, t50: F, t82510: F, t10469: F, t461: F, t11721: F, t3032: F, t3508: F, t7368: F, t11553: F, t2121: F, t2148: F, t27561: F, t7327: F, t210: F, t24810: F, t24848: F, t1090: F, t24815: F, t24594: F, t24847: F, t974: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t85952, t85963, t85964, t85966, t85972, t85986) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2024::<F>(t7365, t85660, t131, t467, t50, t82510, t10469, t461, t11721, t3032, t3508, t7368);
        let (t86000, t86015, t86036, t86037, t86039, t86076) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2025::<F>(t11553, t2121, t2148, t27561, t7327, t210, t24810, t24848, t1090, t24815, t24594, t24847, t974);
    (t85952, t85963, t85964, t85966, t85972, t85986, t86000, t86015, t86036, t86037, t86039, t86076)
}
