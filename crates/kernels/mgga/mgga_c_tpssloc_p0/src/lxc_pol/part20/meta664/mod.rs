//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta664 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2485;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2486;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2487;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2488;
use chunk4::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2489;
use chunk5::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2490;
use chunk6::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2491;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta664<F: Float>(t14730: F, t9288: F, t1113: F, t136: F, t12606: F, t3242: F, t607: F, t3297: F, t123: F, t3240: F, t50857: F, t50861: F, t43835: F, t43837: F, t43839: F, t43855: F, t43857: F, t43859: F, t43861: F, t43863: F, t14750: F, t690: F, t14754: F, t14745: F, t11147: F, t2244: F, t3966: F, t11145: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50879, t50881, t50884, t50886, t50897) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2485::<F>(t14730, t9288, t1113, t136, t12606, t3242, t607, t3297, t123, t3240, t50857);
        let t50900 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2486::<F>(t123, t3240, t50861);
        let t50902 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2487::<F>(t43835, t43837, t43839, t43855, t43857, t43859, t43861, t43863, t50881, t50886, t50897, t50900);
        let t50903 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2488::<F>(t14750, t690);
        let t50905 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2489::<F>(t14754, t690);
        let t50907 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2490::<F>(t14745, t690);
        let (t50910, t50912) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2491::<F>(t11147, t2244, t3966, t11145, t123);
    (t50879, t50881, t50884, t50886, t50897, t50900, t50902, t50903, t50905, t50907, t50910, t50912)
}
