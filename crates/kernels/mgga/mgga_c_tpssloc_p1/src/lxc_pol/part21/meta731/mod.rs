//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta731 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2587;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2588;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta731<F: Float>(t3447: F, t44583: F, t461: F, t4729: F, t15418: F, t1714: F, t11571: F, t14736: F, t15419: F, t14165: F, t44505: F, t11557: F, t4889: F, t11560: F, t1174: F, t1716: F, t2402: F, t4930: F, t698: F, t11513: F, t11589: F, t15313: F, t14749: F, t15402: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t52057, t52059, t52061, t52064, t52066, t52074) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2587::<F>(t3447, t44583, t461, t4729, t15418, t1714, t11571, t14736, t15419, t14165, t44505, t11557, t4889);
        let (t52076, t52081, t52084, t52086, t52089, t52092) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2588::<F>(t11560, t4889, t1174, t1716, t2402, t4930, t698, t11513, t11589, t15313, t3447, t14749, t15402);
    (t52057, t52059, t52061, t52064, t52066, t52074, t52076, t52081, t52084, t52086, t52089, t52092)
}
