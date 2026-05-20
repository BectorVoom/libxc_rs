//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1942;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1943;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta572<F: Float>(t28593: F, t383: F, t1058: F, t1920: F, t23619: F, t25465: F, t25508: F, t28597: F, t28602: F, t28605: F, t28610: F, t28614: F, t28618: F, t28622: F, t28626: F, t28631: F, t3200: F, t353: F, t4669: F, t6687: F, t6797: F, t7620: F, t5677: F, t6785: F, t23696: F, t1945: F, t5866: F, t1060: F, t25470: F, t7603: F, t1409: F, t1615: F, t6800: F, t23635: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t28634, t28636) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1942::<F>(t28593, t383, t1058, t1920, t23619, t25465, t25508, t28597, t28602, t28605, t28610, t28614, t28618, t28622, t28626, t28631, t3200, t353, t4669, t6687, t6797, t7620);
        let (t28637, t28638, t28641, t28642, t28648, t28651, t28652, t28653) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1943::<F>(t5677, t6785, t23696, t1945, t5866, t1060, t25470, t7603, t1409, t1615, t6800, t23635);
    (t28634, t28636, t28637, t28638, t28641, t28642, t28648, t28651, t28652, t28653)
}
