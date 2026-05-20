//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta624 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2104;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2105;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta624<F: Float>(t12521: F, t7467: F, t81440: F, t1453: F, t81439: F, t26129: F, t81442: F, t22470: F, t4067: F, t2332: F, t81446: F, t666: F, t22473: F, t2358: F, t12808: F, t6530: F, t81438: F, t81443: F, t81445: F, t109: F, t1401: F, t55571: F, t7769: F, t20173: F, t26542: F, t26545: F, t12524: F, t1458: F, t22479: F, t3941: F, t4072: F, t6534: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t86582, t86583, t86586, t86589, t86591, t86593, t86595) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2104::<F>(t12521, t7467, t81440, t1453, t81439, t26129, t81442, t22470, t4067, t2332, t81446, t666);
        let t86603 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2105::<F>(t22473, t86595, t1453, t2358, t12808, t6530, t81438, t81443, t81445, t86583, t86586, t86589, t86591, t86593);
        let (t86604, t86606, t86610, t86612, t86614, t86616, t86619, t86622) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2106::<F>(t109, t86603, t1401, t55571, t7769, t20173, t26542, t26545, t12524, t1458, t22479, t3941, t4072, t6534);
    (t86582, t86604, t86606, t86610, t86612, t86614, t86616, t86619, t86622)
}
