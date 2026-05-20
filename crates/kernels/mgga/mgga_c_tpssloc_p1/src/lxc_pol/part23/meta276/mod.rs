//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta276 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk964;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk965;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk966;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta276<F: Float>(t40: F, t52: F, t20217: F, t20234: F, t4080: F, t5398: F, t73: F, t9427: F, t4087: F, t76: F, t9438: F, t157: F, t182: F, t16587: F, zeta_threshold: F, t4195: F, t4194: F, t1530: F, t17116: F, t1877: F, t20723: F, t20724: F, t9457: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F, t9724: F, t4310: F, t5527: F, t1484: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t20741, t20742, t20744, t20745) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk964::<F>(t40, t52, t20217, t20234, t4080, t5398, t73, t9427, t4087, t76, t9438, t157, t182, t16587, zeta_threshold);
        let (t20749, t20751, t20752) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk965::<F>(t4195, t5398, t4194, t1530, t17116, t1877, t20723, t20724, t20744, t20745, t9457, t9469, t9476, t9484, t9496, t9715, t9724);
        let (t20753, t20756) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk966::<F>(t4310, t5527, t1484);
    (t20741, t20742, t20744, t20745, t20749, t20751, t20752, t20753, t20756)
}
