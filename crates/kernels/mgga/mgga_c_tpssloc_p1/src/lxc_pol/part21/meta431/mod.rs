//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta431 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1964;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1965;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta431<F: Float>(t15320: F, t3451: F, t11579: F, t4919: F, t11584: F, t1174: F, t15294: F, t15300: F, t15304: F, t15307: F, t15314: F, t15317: F, t3443: F, t3447: F, t3457: F, t3461: F, t4889: F, t14753: F, t4908: F, t14744: F, t11588: F, t1714: F) -> (F, F, F, F, F, F, F) {
        let (t15321, t15324, t15327, t15330) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1964::<F>(t15320, t3451, t11579, t4919, t11584, t1174, t15294, t15300, t15304, t15307, t15314, t15317, t3443, t3447, t3457, t3461, t4889);
        let (t15332, t15335, t15338) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1965::<F>(t14753, t4908, t14744, t11588, t1714);
    (t15321, t15324, t15327, t15330, t15332, t15335, t15338)
}
