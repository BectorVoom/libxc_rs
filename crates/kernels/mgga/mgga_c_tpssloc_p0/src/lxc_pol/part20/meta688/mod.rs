//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta688 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2605;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2606;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2607;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2608;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2609;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2610;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta688<F: Float>(t3575: F, t373: F, t470: F, t493: F, t1214: F, t820: F, t3624: F, t52627: F, t11745: F, t15503: F, t15737: F, t3493: F, t475: F, t607: F, t1227: F, t13969: F, t15649: F, t43763: F, t44827: F, t11539: F, t1174: F, t14740: F, t11694: F, t11739: F, t1215: F, t1653: F, t3577: F, t3578: F, t44936: F, t45119: F, t45128: F, t45197: F, t4582: F, t4728: F, t48497: F, t52183: F, t52704: F, t14731: F, t135: F, t15666: F, t11665: F, t15572: F, t3515: F, t4983: F, t49850: F, t11818: F, t1213: F, t248: F, t5012: F, t11801: F, t5024: F, t11820: F, t5019: F, t11729: F, t15527: F, t15541: F, t15545: F, t15656: F, t3490: F, t3536: F, t44836: F, t45037: F, t45997: F, t46006: F, t4977: F, t4987: F, t11791: F, t5002: F, t11153: F, t4899: F, t3540: F, t4961: F, t11709: F, t15640: F, t11738: F, t15535: F, t15553: F, t3447: F, t3516: F, t44965: F, t44968: F, t44972: F, t44976: F, t44982: F, t45971: F, t15611: F, t15454: F, t4973: F, t11662: F, t15478: F, t44985: F, t44988: F, t44991: F, t44994: F, t44996: F, t4950: F, t51002: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t52893, t52897, t52903, t52906, t52908, t52911) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2605::<F>(t3575, t373, t470, t493, t1214, t820, t3624, t52627, t11745, t15503, t15737, t3493, t475, t607);
        let t52928 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2606::<F>(t1227, t13969, t15649, t43763, t44827, t11539, t1174, t14740, t11694, t11739, t1215, t1653, t3493, t3577, t3578, t44936, t45119, t45128, t45197, t4582, t4728, t48497, t52183, t52704, t52893, t52897, t52903, t52906, t52908, t52911);
        let (t52932, t52935, t52942, t52953, t52973) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2607::<F>(t11539, t1174, t14731, t135, t15666, t11665, t15572, t3515, t4983, t49850, t11818, t1213, t248, t5012);
        let t52989 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2608::<F>(t52973, t11801, t5024, t11820, t5019, t11729, t11739, t1227, t15527, t15541, t15545, t15656, t3490, t3536, t44836, t45037, t4582, t45997, t46006, t4977, t4987);
        let t53013 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2609::<F>(t11791, t5024, t11820, t5002, t11153, t4899, t3540, t4961, t11709, t15640, t11738, t15535, t15553, t3447, t3516, t44965, t44968, t44972, t44976, t44982, t4582, t45971);
        let t53037 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2610::<F>(t1227, t13969, t15611, t15454, t4973, t49850, t11662, t11665, t15478, t15737, t44985, t44988, t44991, t44994, t44996, t4582, t48497, t4950, t51002);
    (t52893, t52897, t52911, t52928, t52932, t52935, t52942, t52953, t52989, t53013, t53037)
}
