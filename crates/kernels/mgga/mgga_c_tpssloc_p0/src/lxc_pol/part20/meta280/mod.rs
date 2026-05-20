//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta280 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1466;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1467;
use chunk2::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1468;
use chunk3::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1469;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta280<F: Float>(t10619: F, t324: F, t300: F, t2897: F, t961: F, t2940: F, t2948: F, t2928: F, t941: F, t2931: F, t323: F, t10524: F, t959: F, t10544: F, t10530: F, t10538: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10620, t10622, t10623) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1466::<F>(t10619, t324, t300, t2897);
        let (t10625, t10627, t10629) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1467::<F>(t10623, t961, t2940, t2948, t2928, t941);
        let t10632 = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1468::<F>(t2931, t323);
        let (t10633, t10635, t10636, t10647) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1469::<F>(t10524, t10629, t10632, t959, t10544, t10530, t10538, t10556, t10558, t10560, t10562, t10566, t10569, t10572, t10575);
    (t10620, t10622, t10623, t10625, t10627, t10629, t10632, t10633, t10635, t10636, t10647)
}
