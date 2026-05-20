//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta633 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1895;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1896;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta633<F: Float>(t1361: F, t22690: F, t6330: F, t80840: F, t22792: F, t6347: F, t26318: F, t7708: F, t91351: F, t19844: F, t6916: F, t22804: F, t28077: F, t22779: F, t28067: F, t19924: F, t26288: F, t19994: F, t19919: F, t221: F, t91194: F, t26284: F, t91198: F, t20000: F, t91361: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t97427, t97431, t97435, t97437, t97439) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1895::<F>(t1361, t22690, t6330, t80840, t22792, t6347, t26318, t7708, t91351, t19844, t6916, t22804, t28077);
        let (t97444, t97447, t97450, t97453, t97456, t97459, t97461) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1896::<F>(t22779, t28067, t1361, t19924, t26288, t19994, t19919, t221, t91194, t26284, t91198, t20000, t91361);
    (t97427, t97431, t97435, t97437, t97439, t97444, t97447, t97450, t97453, t97456, t97459, t97461)
}
