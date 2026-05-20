//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta277 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk967;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk968;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk969;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk970;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta277<F: Float>(t16617: F, t12943: F, t16630: F, t12946: F, t145: F, t20741: F, t185: F, t4315: F, t5544: F, t1484: F, t16606: F, t193: F, t20753: F, t20756: F, t2522: F, t262: F, t4314: F, t9780: F, t9789: F, t9793: F, t9797: F, t9863: F, t40: F, t52: F, t13107: F, t1530: F, t5664: F, t20217: F, t20234: F, t4104: F, t5398: F, t634: F, t767: F, t4111: F, t638: F, t771: F, zeta_threshold: F, t1510: F, t17027: F, t20723: F, t20724: F, t20744: F, t20745: F, t20751: F, t9457: F, t9469: F, t9476: F, t9484: F, t9496: F, t9715: F, t9724: F, t4205: F, t5597: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t20760, t20761, t20765, t20766, t20767, t20768, t20772) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk967::<F>(t16617, t12943, t16630, t12946, t145, t20741, t185, t4315, t5544, t1484, t16606, t193, t20753, t20756, t2522, t262, t4314, t9780, t9789, t9793, t9797, t9863);
        let (t20777, t20778, t20790, t20798) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk968::<F>(t40, t52, t13107, t1530, t5664, t20217, t20234, t4104, t5398, t634, t767, t4111, t638, t771, zeta_threshold);
        let t20800 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk969::<F>(t20790, t20798);
        let (t20806, t20811) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk970::<F>(t1510, t17027, t20723, t20724, t20744, t20745, t20751, t9457, t9469, t9476, t9484, t9496, t9715);
        let (t20812, t20815) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk971::<F>(t20760, t20761, t20765, t20766, t20768, t9724, t9780, t9789, t9793, t9797, t9863, t4205, t5597);
    (t20767, t20772, t20777, t20778, t20800, t20806, t20811, t20812, t20815)
}
