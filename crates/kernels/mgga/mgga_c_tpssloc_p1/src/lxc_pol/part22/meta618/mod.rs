//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta618 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2149;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta618<F: Float>(t3610: F, t52627: F, t1227: F, t1653: F, t248: F, t45293: F, t15730: F, t3536: F, t3577: F, t44951: F, t4953: F, t11677: F, t15245: F) -> (F, F, F, F, F) {
        let (t52628, t52680, t52732, t52759, t52766) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2149::<F>(t3610, t52627, t1227, t1653, t248, t45293, t15730, t3536, t3577, t44951, t4953, t11677, t15245);
    (t52628, t52680, t52732, t52759, t52766)
}
