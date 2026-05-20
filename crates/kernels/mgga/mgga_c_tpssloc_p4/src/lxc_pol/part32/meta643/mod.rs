//! MGGA_C_TPSSLOC lxc pol kernel — _part32_v4rho3sigma_8 meta643 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2061;
use chunk1::mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2062;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_meta643<F: Float>(t90514: F, t1377: F, t5187: F, t7692: F, t81186: F, t26338: F, t81228: F, t81326: F, t22892: F, t7691: F, t80645: F, t26206: F, t6883: F, t1834: F, t794: F, t6891: F, t22704: F, t26355: F, t26197: F, t80670: F, t213: F, t225: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t90515, t90516, t90521, t90525, t90534, t90541) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2061::<F>(t90514, t1377, t5187, t7692, t81186, t26338, t81228, t81326, t22892, t7691, t80645, t26206, t6883);
        let (t90542, t90544, t90547, t90550, t90551, t90566) = mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2062::<F>(t90541, t1834, t794, t22892, t6891, t22704, t26355, t81326, t26197, t80670, t213, t225);
    (t90515, t90516, t90521, t90525, t90534, t90542, t90544, t90547, t90550, t90551, t90566)
}
