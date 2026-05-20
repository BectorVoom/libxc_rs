//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta585 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1875;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1876;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta585<F: Float>(t22893: F, t23164: F, t25320: F, t1888: F, t232: F, t47528: F, t6646: F, t13398: F, t82018: F, t13404: F, t22996: F, t7521: F, t81632: F, t23035: F, t2379: F, t25319: F, t6637: F, t1887: F, t81959: F, t25248: F, t25249: F, t4265: F, t828: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t87618, t87627, t87630, t87633, t87635) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1875::<F>(t22893, t23164, t25320, t1888, t232, t47528, t6646, t13398, t82018, t13404, t22996, t7521, t81632);
        let (t87640, t87642, t87645, t87650) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1876::<F>(t23035, t2379, t25319, t6637, t1887, t81959, t25248, t25249, t1888, t232, t4265, t6646, t828);
    (t87618, t87627, t87630, t87633, t87635, t87640, t87642, t87645, t87650)
}
