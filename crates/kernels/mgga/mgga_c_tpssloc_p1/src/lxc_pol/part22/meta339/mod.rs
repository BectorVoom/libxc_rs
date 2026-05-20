//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta339 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1534;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1535;
use chunk2::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1536;
use chunk3::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1537;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta339<F: Float>(t40: F, t12943: F, t4101: F, t4205: F, t4202: F, t16558: F, t185: F, t707: F, t5392: F, t634: F, t5398: F, t75: F, t3966: F, t4104: F, t607: F, t767: F, zeta_threshold: F, t52: F, t638: F, t78: F, t4111: F, t771: F, t12922: F, t12926: F, t12934: F, t16612: F, t16618: F, t16622: F, t16623: F, t16624: F, t16625: F, t193: F, t2522: F, t4255: F, t4310: F, t4314: F, t766: F, t776: F, t9715: F, t9724: F, t9726: F, t9780: F, t9863: F, t5575: F, t68: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t16629, t16630, t16631, t16633, t16634, t16636, t16637, t16648) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1534::<F>(t40, t12943, t4101, t4205, t4202, t16558, t185, t707, t5392, t634, t5398, t75, t3966, t4104, t607, t767, zeta_threshold);
        let (t16649, t16662) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1535::<F>(t52, t5392, t638, t5398, t78, t16558, t3966, t4111, t607, t771, t16648, zeta_threshold);
        let t16666 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1536::<F>(t12922, t12926, t12934, t16612, t16618, t16622, t16623, t16624, t16625, t16629, t16631, t16633, t16636, t16662, t193, t2522, t4255, t4310, t4314, t766, t776, t9715, t9724, t9726, t9780, t9863);
        let t16673 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1537::<F>(t5575, t68);
    (t16629, t16630, t16631, t16633, t16634, t16636, t16637, t16649, t16662, t16666, t16673)
}
