//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta305 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1371;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1372;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta305<F: Float>(t2781: F, t690: F, t154: F, t3061: F, t10544: F, t276: F, t285: F, t273: F, t2897: F, t300: F, t2928: F, t941: F, t2931: F, t323: F) -> (F, F, F, F, F, F, F, F, F) {
        let t10562 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1371::<F>(t2781, t690);
        let (t10564, t10577, t10595, t10599, t10608, t10623, t10629, t10632) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1372::<F>(t154, t3061, t10544, t276, t285, t273, t2897, t300, t2928, t941, t2931, t323);
    (t10562, t10564, t10577, t10595, t10599, t10608, t10623, t10629, t10632)
}
