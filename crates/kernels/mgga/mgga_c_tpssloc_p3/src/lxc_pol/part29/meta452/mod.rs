//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta452 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1768;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1769;
use chunk2::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1770;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta452<F: Float>(t252: F, t2631: F, t2632: F, t22996: F, t1888: F, t6579: F, t6649: F, t232: F, t6646: F, t1902: F, t2627: F, t2633: F, t1879: F, t22715: F, t1906: F, t2679: F, t6657: F, t1894: F, t2710: F, t214: F, t1880: F, t1909: F, t22984: F, t22990: F, t22993: F, t2613: F, t2617: F, t6658: F, t6660: F, t808: F, t812: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t22998, t22999, t23000, t23002, t23003, t23004, t23005, t23006, t23009) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1768::<F>(t252, t2631, t2632, t22996, t1888, t6579, t6649, t232, t6646, t1902, t2627, t2633);
        let t23012 = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1769::<F>(t1879, t22715);
        let (t23014, t23016, t23020, t23021, t23024) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1770::<F>(t1906, t23012, t2679, t6657, t1894, t2710, t214, t1880, t1909, t22984, t22990, t22993, t23000, t23003, t23006, t23009, t2613, t2617, t6658, t6660, t808, t812);
    (t22998, t22999, t23002, t23004, t23005, t23009, t23012, t23014, t23016, t23020, t23021, t23024)
}
