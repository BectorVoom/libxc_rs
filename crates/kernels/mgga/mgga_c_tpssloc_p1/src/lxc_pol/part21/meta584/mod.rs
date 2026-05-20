//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2314;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2315;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2316;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta584<F: Float>(t19735: F, t5335: F, t1824: F, t1834: F, t5250: F, t562: F, t6387: F, t12250: F, t1351: F, t5287: F, t5348: F, t1336: F, t16047: F, t19654: F, t19658: F, t19661: F, t19668: F, t19674: F, t19733: F, t3777: F, t5234: F, t5334: F, t5336: F, t5349: F, t6448: F, t6451: F, t6454: F, t6456: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t19736, t19739) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2314::<F>(t19735, t5335, t1824, t1834);
        let (t19740, t19743) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2315::<F>(t19739, t5250, t562, t6387);
        let (t19744, t19745, t19748, t19752, t19755) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2316::<F>(t12250, t1351, t19743, t5250, t5287, t5348, t1336, t16047, t19654, t19658, t19661, t19668, t19674, t19733, t19736, t19740, t3777, t5234, t5334, t5336, t5349, t6448, t6451, t6454, t6456);
    (t19736, t19739, t19740, t19743, t19744, t19745, t19748, t19752, t19755)
}
