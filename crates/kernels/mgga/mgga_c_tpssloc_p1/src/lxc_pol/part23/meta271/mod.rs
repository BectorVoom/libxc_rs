//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk950;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk951;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta271<F: Float>(t15979: F, t15982: F, t15984: F, t15986: F, t16164: F, t184: F, t20396: F, t17: F, t12118: F, t12121: F, t12123: F, t12133: F, t12141: F, t9853: F, t9859: F, t20519: F, t20521: F, t20525: F, t225: F, t12155: F, t20356: F, t5279: F, t6347: F, t1347: F, t20416: F, t1819: F, t1821: F, t5278: F, t546: F, t548: F, t6404: F, t6408: F, t6411: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t20526, t20527, t20528, t20529, t20530, t20531, t20532, t20533) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk950::<F>(t15979, t15982, t15984, t15986, t16164, t184, t20396, t17, t12118, t12121, t12123, t12133, t12141, t9853, t9859);
        let (t20536, t20544, t20547, t20550, t20553) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk951::<F>(t20519, t20521, t20525, t20533, t225, t12155, t20356, t5279, t6347, t1347, t20416, t1819, t1821, t5278, t546, t548, t6404, t6408, t6411);
    (t20526, t20527, t20528, t20529, t20530, t20531, t20532, t20536, t20544, t20547, t20550, t20553)
}
