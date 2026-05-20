//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta503 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1941;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1942;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta503<F: Float>(t1670: F, t5988: F, t1118: F, t3313: F, t14838: F, t5989: F, t1703: F, t18915: F, t4869: F, t6098: F, t4748: F, t5999: F, t4764: F, t4723: F, t5398: F, t3297: F, t136: F, t4728: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t21723, t21724, t21726, t21728, t21730, t21732, t21739) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1941::<F>(t1670, t5988, t1118, t3313, t14838, t5989, t1703, t18915, t4869, t6098, t4748, t5999);
        let (t21741, t21745) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1942::<F>(t4764, t5999, t4723, t5398);
        let (t21746, t21747, t21749) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1943::<F>(t21745, t3297, t136, t4728, t5398);
    (t21723, t21724, t21726, t21728, t21730, t21732, t21739, t21741, t21745, t21746, t21747, t21749)
}
