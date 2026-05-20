//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta578 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1814;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1815;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta578<F: Float>(t26339: F, t81159: F, t22716: F, t7697: F, t26216: F, t26210: F, t6897: F, t794: F, t1377: F, t5187: F, t7692: F, t81186: F, t26338: F, t81228: F, t81326: F, t22892: F, t7691: F, t80645: F, t26206: F, t6883: F, t1834: F, t6891: F, t22704: F, t26355: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t90500, t90503, t90511, t90514, t90516, t90521) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1814::<F>(t26339, t81159, t22716, t7697, t26216, t26210, t6897, t794, t1377, t5187, t7692, t81186);
        let (t90524, t90533, t90541, t90544, t90546, t90549) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1815::<F>(t26338, t81228, t81326, t22892, t7691, t80645, t26206, t6883, t1834, t794, t6891, t22704, t26355);
    (t90500, t90503, t90511, t90514, t90516, t90521, t90524, t90533, t90541, t90544, t90546, t90549)
}
