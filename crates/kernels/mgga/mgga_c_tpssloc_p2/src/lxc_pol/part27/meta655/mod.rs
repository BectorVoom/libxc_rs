//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta655 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2286;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2287;
use chunk2::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2288;
use chunk3::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2289;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta655<F: Float>(t22666: F, t26189: F, t6888: F, t22892: F, t7691: F, t80645: F, t22633: F, t22635: F, t26337: F, t3911: F, t26206: F, t6883: F, t1834: F, t794: F, t6891: F, t22704: F, t26355: F, t81326: F, t26197: F, t80670: F, t1307: F, t26331: F, t5187: F, t567: F, t26332: F, t3719: F, t213: F, t225: F, t22637: F, t26333: F, t80650: F, t16470: F, t26224: F, t26225: F, t80689: F, t80711: F, t22724: F, t26474: F, t22751: F, t26194: F, t1887: F, t80830: F, t3734: F, t22916: F, t26193: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t90530, t90534, t90539, t90541) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2286::<F>(t22666, t26189, t6888, t22892, t7691, t80645, t22633, t22635, t26337, t3911, t26206, t6883);
        let (t90542, t90544, t90547, t90550, t90551, t90556) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2287::<F>(t90541, t1834, t794, t22892, t6891, t22704, t26355, t81326, t26197, t80670, t1307, t22635, t26331, t5187, t567);
        let t90573 = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2288::<F>(t22635, t26331, t26332, t3719, t1834, t213, t225, t22633, t22637, t26333, t80650, t16470, t26224, t26225, t80689, t90539, t90542, t90547, t90550, t90551, t90556);
        let (t90581, t90582, t90585, t90591, t90594, t90598) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2289::<F>(t80711, t22724, t26474, t22751, t26194, t1887, t80830, t22635, t26332, t3734, t22916, t26193, t6888);
    (t90530, t90534, t90544, t90573, t90581, t90582, t90585, t90591, t90594, t90598)
}
