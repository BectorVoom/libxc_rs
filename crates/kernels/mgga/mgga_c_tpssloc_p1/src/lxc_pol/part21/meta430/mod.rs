//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta430 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1962;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1963;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta430<F: Float>(t15293: F, t3449: F, t11529: F, t1709: F, t1174: F, t1714: F, t3475: F, t460: F, t4934: F, t3432: F, t4889: F, t3450: F, t3966: F, t14749: F, t4908: F, t3448: F, t4928: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t15294, t15299, t15300, t15303, t15304, t15307, t15313) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1962::<F>(t15293, t3449, t11529, t1709, t1174, t1714, t3475, t460, t4934, t3432, t4889, t3450, t3966);
        let (t15314, t15317, t15320) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1963::<F>(t15313, t3449, t14749, t4908, t3448, t4928);
    (t15294, t15299, t15300, t15303, t15304, t15307, t15313, t15314, t15317, t15320)
}
