//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta499 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2007;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta499<F: Float>(t1484: F, t868: F, t13115: F, t157: F, t1504: F, t68: F, t1499: F, t4290: F, t4166: F, t4177: F, t2632: F, t4233: F, t4280: F, t3131: F, t4649: F, t1539: F, t6733: F, t3508: F, t5011: F, t1441: F, t671: F, t1388: F, t1799: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16596, t16693, t16729, t16830, t16836, t16935) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2007::<F>(t1484, t868, t13115, t157, t1504, t68, t1499, t4290, t4166, t4177, t2632, t4233);
        let (t17034, t17732, t17748, t18946, t19456, t19577) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2008::<F>(t1499, t4280, t3131, t4649, t1539, t6733, t3508, t5011, t1441, t671, t1388, t1799);
    (t16596, t16693, t16729, t16830, t16836, t16935, t17034, t17732, t17748, t18946, t19456, t19577)
}
