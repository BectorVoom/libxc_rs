//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta526 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1935;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta526<F: Float>(t25749: F, t884: F, t23329: F, t225: F, t7594: F, t254: F, t382: F, t10164: F, t1955: F, t4664: F, t1052: F, t1066: F, t14529: F, t1635: F, t1956: F, t23327: F, t23346: F, t23359: F, t23372: F, t25447: F, t25450: F, t25453: F, t25732: F, t25736: F, t25739: F, t25743: F, t3026: F, t6687: F, t7557: F, t7600: F) -> (F, F, F, F, F, F, F) {
        let (t25750, t25751, t25755, t25757, t25758, t25759, t25762) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1935::<F>(t25749, t884, t23329, t225, t7594, t254, t382, t10164, t1955, t4664, t1052, t1066, t14529, t1635, t1956, t23327, t23346, t23359, t23372, t25447, t25450, t25453, t25732, t25736, t25739, t25743, t3026, t6687, t7557, t7600);
    (t25750, t25751, t25755, t25757, t25758, t25759, t25762)
}
