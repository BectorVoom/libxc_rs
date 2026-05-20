//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta432 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta432<F: Float>(t1060: F, t14595: F, t4673: F, t4677: F, t1625: F, t3120: F, t14506: F, t3199: F, t1058: F, t11034: F, t11051: F, t11059: F, t11065: F, t14572: F, t14574: F, t14578: F, t14581: F, t14587: F, t14591: F, t1630: F, t1632: F, t3076: F, t3180: F, t3186: F, t3193: F, t3200: F, t3202: F, t4669: F, t4674: F, t4678: F, t4681: F) -> (F, F, F, F, F, F) {
        let (t14596, t14600, t14605, t14606, t14608, t14613) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1852::<F>(t1060, t14595, t4673, t4677, t1625, t3120, t14506, t3199, t1058, t11034, t11051, t11059, t11065, t14572, t14574, t14578, t14581, t14587, t14591, t1630, t1632, t3076, t3180, t3186, t3193, t3200, t3202, t4669, t4674, t4678, t4681);
    (t14596, t14600, t14605, t14606, t14608, t14613)
}
