//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta579 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2303;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2304;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta579<F: Float>(t25: F, t3701: F, t6463: F, t15909: F, t5127: F, t5187: F, t11987: F, t6305: F, t3704: F, t5397: F, t1298: F, t16557: F, t2219: F, t5170: F, t606: F, zeta_threshold: F, t28: F, t12000: F, t6312: F, t3711: F, t5966: F, t1081: F, t1302: F, t18196: F, t5178: F) -> (F, F, F, F, F, F, F, F) {
        let (t19596, t19599, t19603, t19606, t19611, t19617) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2303::<F>(t25, t3701, t6463, t15909, t5127, t5187, t11987, t6305, t3704, t5397, t1298, t16557, t2219, t5170, t606, zeta_threshold);
        let (t19618, t19623, t19631) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2304::<F>(t28, t12000, t6312, t3711, t5966, t1081, t1302, t18196, t2219, t5178, t19617, zeta_threshold);
    (t19596, t19599, t19603, t19606, t19611, t19618, t19623, t19631)
}
