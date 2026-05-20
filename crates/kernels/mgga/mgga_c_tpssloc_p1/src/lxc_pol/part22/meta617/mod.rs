//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2147;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2148;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta617<F: Float>(t52367: F, t3030: F, t4940: F, t3623: F, t11712: F, t11880: F, t491: F, t1734: F, t6739: F, t3609: F, t3242: F, t475: F, t1174: F, t44571: F, t4724: F, t11778: F, t43791: F, t1227: F, t49850: F, t4988: F, t15568: F, t3604: F, t10401: F, t15567: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52368, t52434, t52435, t52479, t52480, t52485, t52548) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2147::<F>(t52367, t3030, t4940, t3623, t11712, t11880, t491, t1734, t6739, t3609, t3242, t475);
        let (t52600, t52601, t52610, t52615, t52627) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2148::<F>(t1174, t44571, t4724, t11778, t43791, t1227, t49850, t4988, t15568, t3604, t10401, t15567);
    (t52368, t52434, t52435, t52479, t52480, t52485, t52548, t52600, t52601, t52610, t52615, t52627)
}
