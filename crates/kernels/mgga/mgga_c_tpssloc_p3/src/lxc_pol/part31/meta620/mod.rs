//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta620 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1871;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1872;
use chunk2::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1873;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta620<F: Float>(t22704: F, t22705: F, t28167: F, t26331: F, t26421: F, t26446: F, t5187: F, t1992: F, t22897: F, t3792: F, t57607: F, t19745: F, t81027: F, t12369: F, t19743: F, t22633: F, t562: F, t6330: F, t1307: F, t90591: F, t20018: F, t6976: F, t550: F, t57499: F, t28163: F, t57618: F, t22881: F, t6347: F, t6637: F, t6888: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t96989, t96993, t96997, t97002) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1871::<F>(t22704, t22705, t28167, t26331, t26421, t26446, t5187, t1992, t22897, t3792, t57607, t19745, t81027);
        let (t97007, t97011, t97014, t97017) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1872::<F>(t12369, t19743, t22633, t22897, t562, t6330, t1307, t26446, t90591, t1992, t20018, t6976);
        let (t97023, t97026, t97030, t97036) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1873::<F>(t1992, t550, t57499, t6976, t22704, t22705, t28163, t57618, t22881, t6347, t6637, t6888);
    (t96989, t96993, t96997, t97002, t97007, t97011, t97014, t97017, t97023, t97026, t97030, t97036)
}
