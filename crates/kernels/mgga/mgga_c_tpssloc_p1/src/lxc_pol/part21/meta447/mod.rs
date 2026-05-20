//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta447 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1995;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta447<F: Float>(t1089: F, t1215: F, t607: F, t15659: F, t3578: F, t1196: F, t12606: F, t974: F, t3548: F, t4889: F, t14736: F, t3440: F, t14740: F, t11678: F, t1174: F, t11755: F, t11787: F, t11792: F, t11794: F, t11798: F, t11802: F, t11821: F, t1227: F, t15650: F, t15656: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t15661, t15662, t15663, t15666, t15667, t15671, t15672) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1995::<F>(t1089, t1215, t607, t15659, t3578, t1196, t12606, t974, t3548, t4889, t14736, t3440);
        let (t15681, t15684) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1996::<F>(t14740, t3440, t11678, t1174, t11755, t11787, t11792, t11794, t11798, t11802, t11821, t1227, t15650, t15656, t15663, t15667, t15671, t15672);
    (t15661, t15662, t15663, t15666, t15667, t15671, t15672, t15681, t15684)
}
