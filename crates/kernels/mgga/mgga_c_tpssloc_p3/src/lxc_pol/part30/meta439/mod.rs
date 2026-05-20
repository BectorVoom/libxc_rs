//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta439 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1683;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1684;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta439<F: Float>(t20118: F, t20147: F, t3: F, t112: F, t6470: F, t576: F, t671: F, t1458: F, t4072: F, t5493: F, t12524: F, t1401: F, t16521: F, t16524: F, t19534: F, t3938: F, t3941: F, t5371: F, t5376: F, t5456: F, t577: F, t3792: F, t6414: F, t2632: F, t5611: F, t111: F, t6514: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20148, t20149, t20162, t20173, t20176, t20181, t20186) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1683::<F>(t20118, t20147, t3, t112, t6470, t576, t671, t1458, t4072, t5493, t12524, t1401, t16521, t16524, t19534, t3938, t3941, t5371, t5376, t5456, t577);
        let (t20473, t20986, t22461) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1684::<F>(t3792, t6414, t2632, t5611, t111, t6514);
    (t20148, t20149, t20162, t20173, t20176, t20181, t20186, t20473, t20986, t22461)
}
