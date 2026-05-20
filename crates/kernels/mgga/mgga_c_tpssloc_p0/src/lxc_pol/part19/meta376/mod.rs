//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta376 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1401;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1402;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1403;
use chunk3::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1404;
use chunk4::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1405;
use chunk5::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1406;
use chunk6::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1407;
use chunk7::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1408;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta376<F: Float>(t43748: F, t43750: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t11778: F, t154: F, t123: F, t43764: F, t1091: F, t9698: F, t22715: F, t268: F, t405: F, t3240: F, t43752: F, t1088: F, t43757: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t1107: F, t11223: F, t699: F, t11205: F, t11208: F, t11219: F, t136: F, t43792: F, t3297: F, t43796: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t43808, t43809) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1401::<F>(t43748, t43750, t43780, t43782, t43784, t43786, t43788, t43794, t43798, t43802, t43806, t11778, t154);
        let t43811 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1402::<F>(t123, t43764, t43809);
        let t43816 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1403::<F>(t1091, t9698);
        let t43819 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1404::<F>(t22715, t268, t405);
        let (t43820, t43823) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1405::<F>(t43819, t123, t3240, t43752);
        let t43828 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1406::<F>(t1088, t123, t43757);
        let t43831 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1407::<F>(t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43811, t43816, t43820, t43823, t43828);
        let (t43832, t43833, t43835, t43837, t43839, t43842, t43845) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1408::<F>(t43808, t43831, t1107, t11223, t699, t11205, t11208, t11219, t136, t43792, t3297, t43796);
    (t43811, t43816, t43819, t43823, t43828, t43832, t43833, t43835, t43837, t43839, t43842, t43845)
}
