//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta126 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk747;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk748;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta126<F: Float>(t2764: F, t273: F, t2799: F, t2807: F, t901: F, t241: F, t63: F, t281: F, t283: F, t699: F, t909: F, t976: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t2810, t2815, t2816, t2818, t2820, t2822, t2823, t2824) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk747::<F>(t2764, t273, t2799, t2807, t901, t241, t63, t281, t283, t699, t909);
        let t2826 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk748::<F>(t241, t976);
    (t2810, t2815, t2816, t2818, t2820, t2822, t2823, t2824, t2826)
}
