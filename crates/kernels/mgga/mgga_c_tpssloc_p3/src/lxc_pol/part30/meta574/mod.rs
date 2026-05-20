//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta574 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1946;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1947;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta574<F: Float>(t28636: F, t28677: F, t1055: F, t1599: F, t7561: F, t25406: F, t7565: F, t1922: F, t5838: F, t1955: F, t5919: F, t10165: F, t1409: F, t1634: F, t23330: F, t23329: F, t25442: F, t7553: F, t5943: F, t3174: F, t1052: F, t17575: F, t17588: F, t18074: F, t1956: F, t23327: F, t23359: F, t25807: F, t25824: F, t28594: F, t388: F, t4557: F, t5920: F, t5944: F, t6687: F, t6771: F, t7600: F, t7625: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t28678, t28679, t28681, t28684, t28691, t28697) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1946::<F>(t28636, t28677, t1055, t1599, t7561, t25406, t7565, t1922, t5838, t1955, t5919, t10165);
        let (t28701, t28702, t28705, t28713, t28718) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1947::<F>(t1409, t1634, t23330, t23329, t25442, t7553, t1955, t5943, t3174, t1052, t17575, t17588, t18074, t1956, t23327, t23359, t25807, t25824, t28594, t28679, t28681, t28684, t28691, t28697, t388, t4557, t5920, t5944, t6687, t6771, t7600, t7625);
    (t28678, t28679, t28681, t28684, t28691, t28697, t28701, t28702, t28705, t28713, t28718)
}
