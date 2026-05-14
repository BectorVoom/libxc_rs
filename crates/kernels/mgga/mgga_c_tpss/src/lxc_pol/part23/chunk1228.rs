//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1228/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk1228<F: Float>(t1149: F, t1587: F, t1887: F, t19106: F, t19115: F, t19129: F, t20854: F, t20856: F, t20863: F, t20865: F, t20868: F, t20874: F, t20878: F, t4300: F, t4323: F, t473: F, t6019: F, t6024: F, t6027: F, t6035: F, t6038: F, t6514: F, t6517: F) -> (F,) {
    let t20881 = -t1149 * t20856 - t1587 * t19106 - t1887 * t20863 + 2.0 * t19115 * t6517 + 2.0 * t19129 * t20878 + t20854 * t473 + 2.0 * t20865 * t6027 - t20868 * t6035 - 6.0 * t20874 * t6024 + 2.0 * t4300 * t6019 - t4323 * t6019 - t6038 * t6514;
    (t20881,)
}
