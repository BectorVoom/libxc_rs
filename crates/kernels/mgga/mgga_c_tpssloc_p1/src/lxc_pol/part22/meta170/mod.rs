//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta170 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1029;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1030;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1031;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1032;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1033;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1034;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta170<F: Float>(t471: F, t5023: F, t1230: F, t248: F, t4733: F, t3440: F, t4724: F, t1193: F, t1706: F, t135: F, t1725: F, t1174: F, t1196: F, t3966: F, t974: F, t1198: F, t1213: F, t1218: F, t1227: F, t1232: F, t1748: F, t3490: F, t3524: F, t3542: F, t3543: F, t3547: F, t3549: F, t3573: F, t4889: F, t5014: F, t5019: F, t5010: F, t466: F, t1752: F, t225: F, t1251: F, t1760: F, t3598: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5024 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1029::<F>(t471, t5023);
        let t5030 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1030::<F>(t1230, t248, t4733);
        let (t5033, t5036, t5040, t5041, t5045, t5046, t5051) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1031::<F>(t3440, t4724, t1193, t1706, t135, t1725, t1174, t1196, t3966, t974, t1198, t1213, t1218, t1227, t1232, t1748, t3490, t3524, t3542, t3543, t3547, t3549, t3573, t4889, t5014, t5019, t5024, t5030);
        let t5052 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1032::<F>(t5010, t5051);
        let (t5053, t5055) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1033::<F>(t466, t5052, t1752, t225);
        let t5060 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1034::<F>(t1251, t1760, t3598);
    (t5024, t5030, t5033, t5036, t5040, t5041, t5045, t5046, t5052, t5053, t5055, t5060)
}
