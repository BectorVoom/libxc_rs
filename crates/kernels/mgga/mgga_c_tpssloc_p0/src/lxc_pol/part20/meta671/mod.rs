//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta671 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2521;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2522;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2523;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2524;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2525;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta671<F: Float>(t11270: F, t4740: F, t11274: F, t1657: F, t11278: F, t1671: F, t43954: F, t11180: F, t4782: F, t14914: F, t3259: F, t1254: F, t15834: F, t3640: F, t4700: F, t50816: F, t50818: F, t50821: F, t51111: F, t51113: F, t11131: F, t4869: F, t11427: F, t14850: F, t50826: F, t43727: F, t43729: F, t43748: F, t43750: F, t50824: F, t50828: F, t50832: F, t50834: F, t50837: F, t50839: F, t50853: F, t43768: F, t43770: F, t44027: F, t50846: F, t50848: F, t50851: F, t50859: F, t50863: F, t50867: F, t50871: F, t50875: F, t43835: F, t43837: F, t43839: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t50881: F, t50886: F, t50897: F, t50900: F, t50903: F, t50905: F, t50907: F, t50912: F, t50917: F, t50919: F, t50921: F, t50926: F, t50931: F, t50934: F, t50937: F, t50940: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t51119, t51122, t51124, t51126, t51128, t51129) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2521::<F>(t11270, t4740, t11274, t1657, t11278, t1671, t43954, t11180, t4782, t14914, t3259, t1254, t15834, t3640, t4700, t50816, t50818, t50821, t51111, t51113);
        let (t51131, t51133, t51147) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2522::<F>(t11131, t4869, t11427, t14850, t50826, t43727, t43729, t43748, t43750, t50824, t50828, t50832, t50834, t50837, t50839);
        let t51159 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2523::<F>(t50853, t43768, t43770, t44027, t50846, t50848, t50851, t50859, t50863, t50867, t50871, t50875);
        let t51173 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2524::<F>(t43835, t43837, t43839, t43855, t43857, t43859, t43861, t43863, t50881, t50886, t50897, t50900);
        let t51186 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2525::<F>(t50903, t50905, t50907, t50912, t50917, t50919, t50921, t50926, t50931, t50934, t50937, t50940);
    (t51119, t51122, t51124, t51126, t51128, t51129, t51131, t51133, t51147, t51159, t51173, t51186)
}
