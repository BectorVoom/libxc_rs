//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta472 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1942;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta472<F: Float>(t1706: F, t3545: F, t11818: F, t1735: F, t248: F, t1213: F, t11789: F, t1653: F, t1227: F, t15437: F, t3505: F, t3576: F, t5064: F) -> (F, F, F, F, F, F, F) {
        let (t15727, t15730, t15731, t15734, t15735, t15737, t15740) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1942::<F>(t1706, t3545, t11818, t1735, t248, t1213, t11789, t1653, t1227, t15437, t3505, t3576, t5064);
    (t15727, t15730, t15731, t15734, t15735, t15737, t15740)
}
