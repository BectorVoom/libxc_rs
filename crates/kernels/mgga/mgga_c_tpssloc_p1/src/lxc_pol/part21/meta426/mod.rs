//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta426 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1951;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1952;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1953;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta426<F: Float>(t3359: F, t4819: F, t1136: F, t3351: F, t4823: F, t11352: F, t1682: F, t3333: F, t1155: F, t4858: F, t1695: F, t3395: F, t3377: F, t4861: F, t14722: F, t14704: F, t11137: F, t11139: F, t11141: F, t11143: F, t11444: F, t14702: F, t14708: F, t14720: F, t14728: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t1675: F, t3331: F, t11297: F, t11350: F, t11361: F, t11365: F, t14958: F, t15048: F, t3334: F, t3357: F, t3376: F, t3401: F, t436: F, t4840: F, t4862: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t15165, t15168, t15172, t15179, t15182) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1951::<F>(t3359, t4819, t1136, t3351, t4823, t11352, t1682, t3333, t1155, t4858, t1695, t3395);
        let (t15185, t15194, t15195, t15204) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1952::<F>(t3377, t4861, t14722, t14704, t11137, t11139, t11141, t11143, t11444, t14702, t14708, t14720, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
        let (t15207, t15210, t15213) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1953::<F>(t1675, t3331, t1695, t3377, t11297, t11350, t11361, t11365, t14958, t15048, t15165, t15168, t15172, t15179, t15182, t15185, t15204, t3334, t3357, t3376, t3401, t436, t4840, t4862);
    (t15165, t15168, t15172, t15179, t15182, t15185, t15194, t15195, t15204, t15207, t15210, t15213)
}
