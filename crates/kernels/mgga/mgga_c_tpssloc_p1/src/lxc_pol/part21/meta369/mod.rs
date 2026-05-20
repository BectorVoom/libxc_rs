//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta369 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1811;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1812;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1813;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1814;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta369<F: Float>(t13563: F, t13566: F, t4348: F, t690: F, t12606: F, t883: F, t882: F, t123: F, t10556: F, t10558: F, t10560: F, t10562: F, t10577: F, t13569: F, t13572: F, t13575: F, t13578: F, t13581: F, t13584: F, t13587: F, t13598: F) -> (F, F, F, F, F, F, F, F) {
        let (t13600, t13601, t13602) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1811::<F>(t13563, t13566, t4348, t690);
        let (t13603, t13611) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1812::<F>(t13602, t12606, t883);
        let (t13612, t13613) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1813::<F>(t13611, t882, t123);
        let t13615 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1814::<F>(t10556, t10558, t10560, t10562, t10577, t13569, t13572, t13575, t13578, t13581, t13584, t13587, t13598, t13600, t13601, t13603, t13613);
    (t13600, t13601, t13602, t13603, t13611, t13612, t13613, t13615)
}
