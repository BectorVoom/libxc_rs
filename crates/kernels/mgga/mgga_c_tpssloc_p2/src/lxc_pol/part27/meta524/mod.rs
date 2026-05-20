//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta524 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1930;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1931;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta524<F: Float>(t360: F, t4649: F, t68: F, t6744: F, t344: F, t7573: F, t6740: F, t1622: F, t23489: F, t23533: F, t23537: F, t23541: F, t23544: F, t23554: F, t23560: F, t4590: F, t4596: F, t4600: F, t4636: F, t4652: F, t6723: F, t6735: F, t6742: F, t6747: F, t6755: F, t6765: F, t7574: F, t7578: F, t7583: F, t25605: F, t25631: F, t25672: F, t383: F, t4673: F, t7619: F, t1598: F, t984: F, t23478: F, t6785: F, t4347: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t25678, t25679, t25682, t25683, t25703) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1930::<F>(t360, t4649, t68, t6744, t344, t7573, t6740, t1622, t23489, t23533, t23537, t23541, t23544, t23554, t23560, t4590, t4596, t4600, t4636, t4652, t6723, t6735, t6742, t6747, t6755, t6765, t7574, t7578, t7583);
        let (t25705, t25706, t25708, t25712, t25713, t25714, t25717) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1931::<F>(t25605, t25631, t25672, t25703, t383, t4673, t7619, t1598, t984, t23478, t6785, t4347);
    (t25678, t25679, t25682, t25683, t25705, t25706, t25708, t25712, t25713, t25714, t25717)
}
