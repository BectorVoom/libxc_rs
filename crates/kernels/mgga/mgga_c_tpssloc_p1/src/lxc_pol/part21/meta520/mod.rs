//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta520 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2171;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta520<F: Float>(t2970: F, t5828: F, t973: F, t16558: F, t978: F, t977: F, t343: F, t5836: F, t984: F, t4546: F, t10231: F, t5817: F, t13861: F, t4531: F, t17178: F, t4510: F, t2989: F, t5398: F, t2988: F, t10186: F, t13830: F, t13850: F, t2960: F, t2986: F, t5818: F, t5821: F, t5829: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t17769, t17770, t17772, t17773, t17777, t17778, t17783) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2171::<F>(t2970, t5828, t973, t16558, t978, t977, t343, t5836, t984, t4546, t10231, t5817);
        let (t17788, t17791, t17794, t17795, t17798) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2172::<F>(t17783, t973, t13861, t4531, t17178, t4510, t2989, t5398, t2988, t10186, t13830, t13850, t17770, t17773, t17778, t2960, t2986, t5818, t5821, t5829);
    (t17769, t17772, t17773, t17777, t17778, t17783, t17788, t17791, t17794, t17795, t17798)
}
