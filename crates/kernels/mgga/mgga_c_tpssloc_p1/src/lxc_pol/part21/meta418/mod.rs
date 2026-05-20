//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1936;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta418<F: Float>(t14933: F, t449: F, t300: F, t1671: F, t3265: F, t3313: F, t14722: F, t14704: F, t11137: F, t11139: F, t11141: F, t11143: F, t11459: F, t14702: F, t14708: F, t14720: F, t14728: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F, t423: F, t1254: F, t14696: F, t14701: F, t14833: F, t14835: F, t14837: F, t14840: F, t14844: F, t14847: F, t14849: F, t14852: F, t14857: F, t14860: F, t14862: F, t14864: F, t14866: F, t14916: F, t4700: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t14934, t14936, t14937, t14939, t14946, t14947, t14956) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1936::<F>(t14933, t449, t300, t1671, t3265, t3313, t14722, t14704, t11137, t11139, t11141, t11143, t11459, t14702, t14708, t14720, t14728, t14733, t14738, t14742, t14746, t14751, t14755);
        let (t14958, t14959) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1937::<F>(t14956, t423, t1254, t14696, t14701, t14833, t14835, t14837, t14840, t14844, t14847, t14849, t14852, t14857, t14860, t14862, t14864, t14866, t14916, t14936, t14939, t4700);
    (t14934, t14936, t14937, t14939, t14946, t14947, t14956, t14958, t14959)
}
