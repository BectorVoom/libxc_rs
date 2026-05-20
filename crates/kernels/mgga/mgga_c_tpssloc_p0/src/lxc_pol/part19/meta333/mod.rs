//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta333 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1194;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1195;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1196;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta333<F: Float>(t39358: F, t756: F, t706: F, t9448: F, t708: F, t187: F, t268: F, t39322: F, t39347: F, t39336: F, t761: F, t2652: F, t9874: F, t2523: F, t39400: F, t39408: F, t39411: F, t39463: F, t39468: F, t39472: F, t39476: F, t4314: F, t9616: F, t751: F, t9288: F, t9897: F, t2244: F, t2517: F, t2658: F, t39488: F, t2531: F, t9919: F, t707: F, t9258: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t40708, t40711, t40714, t40716, t40721, t40722) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1194::<F>(t39358, t756, t706, t9448, t708, t187, t268, t39322, t39347, t39336, t761, t2652, t9874);
        let (t40723, t40724) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1195::<F>(t40722, t2523, t39400, t39408, t39411, t39463, t39468, t39472, t39476, t40708, t40711, t40714, t40716, t40721, t4314, t9616);
        let (t40727, t40730, t40732, t40734, t40736) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1196::<F>(t751, t9288, t9897, t2244, t2517, t2658, t39488, t761, t2531, t9919, t707, t9258);
    (t40708, t40711, t40714, t40716, t40721, t40723, t40724, t40727, t40730, t40732, t40734, t40736)
}
