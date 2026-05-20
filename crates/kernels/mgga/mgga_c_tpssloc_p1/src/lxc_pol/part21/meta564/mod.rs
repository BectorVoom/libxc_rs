//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta564 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2270;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2271;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2272;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta564<F: Float>(t1216: F, t18300: F, t4582: F, t5001: F, t5018: F, t1730: F, t5023: F, t1177: F, t18225: F, t1193: F, t6109: F, t248: F, t3570: F, t6230: F, t3515: F, t1230: F, t18241: F, t11546: F, t18206: F, t11738: F, t1174: F, t1218: F, t1227: F, t1232: F, t15591: F, t15594: F, t15754: F, t1737: F, t1748: F, t3490: F, t4889: F, t5002: F, t5005: F, t5014: F, t5030: F, t5033: F, t6207: F, t6211: F, t18316: F, t18337: F, t18390: F, t18951: F, t18989: F, t19029: F, t19075: F, t466: F, t5068: F, t6260: F, t18940: F, t491: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t19076, t19077, t19080, t19083, t19087, t19090, t19095) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2270::<F>(t1216, t18300, t4582, t5001, t5018, t1730, t5023, t1177, t18225, t1193, t6109, t248, t3570, t6230);
        let (t19101, t19117) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2271::<F>(t19095, t3515, t1230, t18241, t248, t11546, t18206, t11738, t1174, t1218, t1227, t1232, t15591, t15594, t15754, t1737, t1748, t19077, t19080, t19083, t19087, t19090, t3490, t4889, t5002, t5005, t5014, t5030, t5033, t6207, t6211);
        let (t19120, t19121, t19123, t19128) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2272::<F>(t18316, t18337, t18390, t18951, t18989, t19029, t19075, t19117, t466, t5068, t6260, t18940, t491);
    (t19076, t19077, t19080, t19083, t19095, t19101, t19120, t19121, t19123, t19128)
}
