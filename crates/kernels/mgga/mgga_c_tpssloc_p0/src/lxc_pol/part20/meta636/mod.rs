//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta636 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2337;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2338;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2339;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2340;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2341;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta636<F: Float>(t10481: F, t23508: F, t10469: F, t1603: F, t11058: F, t1625: F, t11045: F, t11064: F, t1058: F, t1060: F, t10857: F, t11028: F, t11034: F, t11040: F, t11046: F, t11048: F, t11049: F, t11061: F, t11067: F, t14608: F, t14622: F, t14654: F, t3200: F, t43480: F, t43536: F, t4669: F, t4674: F, t4677: F, t4685: F, t10236: F, t14165: F, t13831: F, t13847: F, t2986: F, t10913: F, t4337: F, t10254: F, t12648: F, t43070: F, t10190: F, t13835: F, t10186: F, t10259: F, t13832: F, t13836: F, t13839: F, t13851: F, t13934: F, t2776: F, t2780: F, t2960: F, t2988: F, t42762: F, t42773: F, t42785: F, t42788: F, t42794: F, t42846: F, t43043: F, t43069: F, t4518: F, t4531: F, t6733: F, t42841: F, t12652: F, t10241: F, t13861: F, t17748: F, t42889: F, t42893: F, t42895: F, t42903: F, t42906: F, t43065: F, t47701: F, t10255: F, t13806: F, t13871: F, t42775: F, t42964: F, t42968: F, t42974: F, t4510: F, t4514: F, t47684: F, t47726: F, t47746: F, t47767: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47819, t47840, t47844, t47867) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2337::<F>(t10481, t23508, t10469, t1603, t11058, t1625, t11045, t11064, t1058, t1060, t10857, t11028, t11034, t11040, t11046, t11048, t11049, t11061, t11067, t14608, t14622, t14654, t3200, t43480, t43536, t4669, t4674, t4677, t4685);
        let (t47887, t47907, t47915, t47919, t47927, t47938) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2338::<F>(t10236, t14165, t13831, t13847, t2986, t10913, t4337, t10254, t12648, t43070, t10190, t13835);
        let t47940 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2339::<F>(t10186, t10259, t13831, t13832, t13835, t13836, t13839, t13851, t13934, t2776, t2780, t2960, t2986, t2988, t42762, t42773, t42785, t42788, t42794, t42846, t43043, t43069, t4518, t4531, t47887, t47907, t47915, t47919, t47927, t47938, t6733);
        let (t47941, t47978) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2340::<F>(t14165, t42841, t10254, t12652, t10241, t10259, t13835, t13839, t13861, t17748, t2986, t2988, t42889, t42893, t42895, t42903, t42906, t43065, t4518, t47701);
        let t48017 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2341::<F>(t10186, t10255, t13806, t13851, t13871, t2986, t42775, t42964, t42968, t42974, t4510, t4514, t4518, t47684, t47726, t47746, t47767);
    (t47819, t47840, t47844, t47867, t47915, t47940, t47941, t47978, t48017)
}
