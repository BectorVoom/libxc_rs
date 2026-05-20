//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2089;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2090;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta547<F: Float>(t212: F, t2553: F, t2586: F, t9523: F, t9525: F, t9577: F, t116: F, t244: F, t2379: F, t2563: F, t9529: F, t207: F, t40419: F, t9538: F, t41083: F, t789: F, t41011: F, t9561: F, t154: F, t1891: F, t205: F, t792: F, t9558: F, t118: F, t794: F, t9458: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t41142, t41144, t41149, t41151, t41155) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2089::<F>(t212, t2553, t2586, t9523, t9525, t9577, t116, t244, t2379, t2563, t9529, t207, t40419, t9538);
        let (t41156, t41158, t41160, t41161, t41173) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2090::<F>(t41083, t789, t41011, t9561, t154, t1891, t205, t792, t9558, t118, t794, t9458);
    (t41142, t41144, t41149, t41151, t41155, t41156, t41158, t41160, t41161, t41173)
}
