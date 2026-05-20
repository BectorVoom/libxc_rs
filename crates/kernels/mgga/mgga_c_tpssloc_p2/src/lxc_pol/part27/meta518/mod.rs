//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta518 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1923;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta518<F: Float>(t25548: F, t6800: F, t23635: F, t1629: F, t6743: F, t884: F, t4684: F, t7619: F, t1610: F, t1920: F, t1953: F, t23633: F, t23666: F, t25530: F, t25536: F, t25541: F, t25545: F, t3200: F, t4615: F, t4669: F, t6797: F, t6811: F, t6813: F) -> (F, F, F, F, F, F, F) {
        let (t25549, t25550, t25553, t25554, t25555, t25558, t25560) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1923::<F>(t25548, t6800, t23635, t1629, t6743, t884, t4684, t7619, t1610, t1920, t1953, t23633, t23666, t25530, t25536, t25541, t25545, t3200, t4615, t4669, t6797, t6811, t6813);
    (t25549, t25550, t25553, t25554, t25555, t25558, t25560)
}
