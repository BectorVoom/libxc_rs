//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta285 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1480;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1481;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1482;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1483;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta285<F: Float>(t10743: F, t932: F, t2904: F, t938: F, t10524: F, t951: F, t10603: F, t10629: F, t315: F, t10632: F, t2853: F, t923: F, t2885: F, t919: F, t10717: F, t10720: F, t10724: F, t10729: F, t10733: F, t10734: F, t10739: F, t10740: F, t2856: F, t2861: F, t2863: F, t2881: F, t2886: F, t2889: F, t2905: F, t2907: F, t2930: F, t933: F, t943: F, t2884: F, t307: F, t302: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10744, t10747, t10750, t10753, t10756) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1480::<F>(t10743, t932, t2904, t938, t10524, t951, t10603, t10629, t315);
        let (t10757, t10760, t10765) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1481::<F>(t10524, t10632, t2853, t923, t2885, t919);
        let t10768 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1482::<F>(t10717, t10720, t10724, t10729, t10733, t10734, t10739, t10740, t10744, t10747, t10750, t10753, t10756, t10757, t10760, t10765, t2856, t2861, t2863, t2881, t2886, t2889, t2905, t2907, t2930, t933, t943);
        let (t10770, t10771) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1483::<F>(t2884, t307, t302);
    (t10744, t10747, t10750, t10753, t10756, t10757, t10760, t10765, t10768, t10770, t10771)
}
