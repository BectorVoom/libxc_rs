//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta169 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk806;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk807;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta169<F: Float>(t135: F, t1606: F, t973: F, t3966: F, t998: F, t974: F, t1041: F, t1607: F, t1622: F, t2960: F, t3039: F, t3048: F, t3054: F, t3070: F, t3084: F, t3092: F, t3130: F, t4562: F, t4565: F, t4572: F, t4575: F, t4579: F, t4585: F, t4590: F, t4596: F, t4600: F, t225: F, t4552: F, t68: F, t369: F, t1031: F, t1611: F, t1036: F, t1612: F, t1616: F, t248: F, t3101: F, t1020: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t4603, t4604, t4608, t4613) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk806::<F>(t135, t1606, t973, t3966, t998, t974, t1041, t1607, t1622, t2960, t3039, t3048, t3054, t3070, t3084, t3092, t3130, t4562, t4565, t4572, t4575, t4579, t4585, t4590, t4596, t4600);
        let (t4615, t4616, t4617, t4622, t4625, t4630, t4631) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk807::<F>(t225, t4552, t68, t369, t1031, t1611, t1036, t1612, t1616, t248, t3101, t1020);
    (t4603, t4604, t4608, t4613, t4615, t4616, t4617, t4622, t4625, t4630, t4631)
}
