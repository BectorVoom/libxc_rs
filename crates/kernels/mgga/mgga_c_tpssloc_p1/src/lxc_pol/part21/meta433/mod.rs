//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta433 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1967;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1968;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1969;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta433<F: Float>(t15357: F, t457: F, t460: F, t974: F, t1716: F, t698: F, t1174: F, t3435: F, t4889: F, t135: F, t4930: F, t1420: F, t1887: F, t337: F, t11593: F, t4904: F, t11570: F, t3961: F, t11569: F, t15332: F, t15335: F, t15341: F, t3447: F, t3452: F, t3472: F, t3478: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15359, t15360, t15363, t15364, t15366, t15372, t15374, t15376) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1967::<F>(t15357, t457, t460, t974, t1716, t698, t1174, t3435, t4889, t135, t4930, t1420, t1887, t337);
        let (t15379, t15382) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1968::<F>(t11593, t4904, t11570, t3961);
        let (t15383, t15386) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1969::<F>(t11569, t15382, t1174, t15332, t15335, t15341, t15360, t15364, t15366, t15374, t15376, t15379, t3447, t3452, t3472, t3478, t4889);
    (t15359, t15360, t15363, t15364, t15366, t15372, t15374, t15376, t15379, t15382, t15383, t15386)
}
