//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta205 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1217;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1218;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1219;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1220;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1221;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1222;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta205<F: Float>(t3961: F, t4987: F, t4582: F, t1653: F, t248: F, t3521: F, t1227: F, t1735: F, t3570: F, t1213: F, t1009: F, t1720: F, t1011: F, t1212: F, t1226: F, t1730: F, t1174: F, t1218: F, t1232: F, t1737: F, t3506: F, t3515: F, t3536: F, t3577: F, t488: F, t4950: F, t4954: F, t4957: F, t4959: F, t4961: F, t4966: F, t4969: F, t4974: F, t4980: F, t4984: F, t4739: F, t4742: F, t4744: F, t4747: F, t4784: F, t4788: F, t4866: F, t4868: F, t4871: F, t4873: F, t4877: F, t4881: F, t4886: F, t475: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t4988, t4989, t4993, t4994, t4997, t4998, t5000) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1217::<F>(t3961, t4987, t4582, t1653, t248, t3521, t1227, t1735, t3570, t1213, t1009, t1720);
        let t5002 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1218::<F>(t1011, t5000, t1212);
        let t5005 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1219::<F>(t1226, t1730);
        let t5010 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1220::<F>(t1174, t1218, t1227, t1232, t1737, t3506, t3515, t3536, t3577, t488, t4950, t4954, t4957, t4959, t4961, t4966, t4969, t4974, t4980, t4984, t4989, t4994, t4998, t5002, t5005);
        let t5011 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1221::<F>(t4739, t4742, t4744, t4747, t4784, t4788, t4866, t4868, t4871, t4873, t4877, t4881, t4886);
        let t5012 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1222::<F>(t475, t5011);
    (t4988, t4989, t4993, t4997, t5000, t5002, t5005, t5010, t5011, t5012)
}
