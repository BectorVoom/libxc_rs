//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta102 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk695;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk696;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk697;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk698;
use chunk4::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk699;
use chunk5::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk700;
use chunk6::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk701;
use chunk7::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk702;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta102<F: Float>(t1878: F, t268: F, t271: F, t690: F, t885: F, t1043: F, t154: F, t632: F, t2289: F, t888: F, t892: F, t287: F, t891: F, t275: F, t273: F, t276: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t2764, t2765, t2766) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk695::<F>(t1878, t268, t271, t690, t885);
        let t2768 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk696::<F>(t1043, t154);
        let t2769 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk697::<F>(t632);
        let t2770 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk698::<F>(t2769);
        let t2775 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk699::<F>(t2289);
        let (t2787, t2790, t2791) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk700::<F>(t888, t892, t287, t891);
        let t2792 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk701::<F>(t275, t2791);
        let t2798 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk702::<F>(t273, t276);
    (t2764, t2765, t2766, t2768, t2769, t2770, t2775, t2787, t2790, t2791, t2792, t2798)
}
