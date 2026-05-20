//! MGGA_C_TPSSLOC lxc pol kernel — _part30_v4rho3sigma_6 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1944;
use chunk1::mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1945;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_meta573<F: Float>(t1949: F, t5844: F, t5838: F, t1599: F, t7614: F, t23678: F, t5928: F, t23677: F, t23604: F, t23603: F, t28596: F, t3188: F, t1058: F, t1610: F, t1953: F, t23327: F, t23601: F, t23633: F, t25530: F, t25563: F, t28638: F, t28642: F, t28648: F, t28653: F, t3186: F, t5903: F, t6687: F, t7622: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t28657, t28660, t28663, t28666, t28667, t28670, t28671, t28674) = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1944::<F>(t1949, t5844, t5838, t1599, t7614, t23678, t5928, t23677, t23604, t23603, t28596, t3188);
        let t28677 = mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1945::<F>(t1058, t1610, t1953, t23327, t23601, t23633, t25530, t25563, t28638, t28642, t28648, t28653, t28657, t28660, t28663, t28667, t28671, t28674, t3186, t5903, t6687, t7622);
    (t28657, t28660, t28663, t28666, t28667, t28670, t28671, t28674, t28677)
}
