//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta568 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1936;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1937;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta568<F: Float>(t25810: F, t7553: F, t5685: F, t6690: F, t6689: F, t1922: F, t5844: F, t1052: F, t1635: F, t1920: F, t25450: F, t25736: F, t25755: F, t25778: F, t28470: F, t28475: F, t28481: F, t28485: F, t28488: F, t28492: F, t28496: F, t28500: F, t28505: F, t388: F, t4660: F, t6687: F, t7600: F, t7625: F, t3: F, t5398: F, t1933: F, t1618: F, t1622: F, t1937: F, t23447: F, t23537: F, t23541: F, t25577: F, t25580: F, t25598: F, t25616: F, t25618: F, t25625: F, t25629: F, t25645: F, t5857: F, t5861: F, t5869: F, t5875: F, t5880: F, t6755: F, t6765: F, t7583: F) -> (F, F, F, F, F, F, F, F) {
        let (t28510, t28515, t28516, t28519, t28523) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1936::<F>(t25810, t7553, t5685, t6690, t6689, t1922, t5844, t1052, t1635, t1920, t25450, t25736, t25755, t25778, t28470, t28475, t28481, t28485, t28488, t28492, t28496, t28500, t28505, t388, t4660, t6687, t7600, t7625);
        let (t28525, t28526, t28550) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1937::<F>(t3, t5398, t1933, t1618, t1622, t1937, t23447, t23537, t23541, t25577, t25580, t25598, t25616, t25618, t25625, t25629, t25645, t5857, t5861, t5869, t5875, t5880, t6755, t6765, t7583);
    (t28510, t28515, t28516, t28519, t28523, t28525, t28526, t28550)
}
