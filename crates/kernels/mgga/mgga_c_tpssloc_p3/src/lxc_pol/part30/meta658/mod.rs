//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta658 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2077;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta658<F: Float>(t90524: F, t22892: F, t7691: F, t80645: F, t26206: F, t6883: F, t1834: F, t794: F, t6891: F, t22704: F, t26355: F, t81326: F, t26197: F, t80670: F, t213: F, t225: F, t22724: F, t26474: F, t22751: F, t26194: F, t1887: F, t80830: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t90525, t90534, t90542, t90544, t90547, t90549) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2077::<F>(t90524, t22892, t7691, t80645, t26206, t6883, t1834, t794, t6891, t22704, t26355, t81326);
        let (t90550, t90551, t90566, t90582, t90585, t90591) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2078::<F>(t90549, t26197, t80670, t1834, t213, t225, t22724, t26474, t22751, t26194, t1887, t80830);
    (t90525, t90534, t90542, t90544, t90547, t90550, t90551, t90566, t90582, t90585, t90591)
}
