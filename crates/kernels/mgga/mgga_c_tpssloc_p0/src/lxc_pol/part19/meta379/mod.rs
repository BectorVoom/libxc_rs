//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta379 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1416;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1417;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1418;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta379<F: Float>(t43819: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43811: F, t43816: F, t43823: F, t43828: F, t43936: F, t449: F, t300: F, t1098: F, t11470: F, t1119: F, t11180: F, t3308: F, t3256: F, t3312: F, t3316: F, t11270: F, t3259: F, t1094: F, t11274: F, t11278: F, t3262: F, t3311: F, t409: F, t3265: F, t11277: F, t11634: F, t3411: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let t43949 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1416::<F>(t43819, t43727, t43729, t43734, t43737, t43740, t43743, t43746, t43811, t43816, t43823, t43828);
        let (t43951, t43953, t43956, t43958, t43961) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1417::<F>(t43936, t43949, t449, t300, t1098, t11470, t1119, t11180, t3308, t3256, t3312, t3316);
        let (t43963, t43966, t43970, t43973, t43975) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1418::<F>(t11270, t3259, t1094, t11274, t11278, t3262, t3311, t409, t3265, t11277, t11634, t3411);
    (t43951, t43953, t43956, t43958, t43961, t43963, t43966, t43970, t43973, t43975)
}
