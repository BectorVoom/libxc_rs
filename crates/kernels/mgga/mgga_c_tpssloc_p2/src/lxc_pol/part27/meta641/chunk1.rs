//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2174/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2174<F: Float>(t23035: F, t2379: F, t25319: F, t6637: F, t1887: F, t81959: F, t25248: F, t25249: F, t1888: F, t232: F, t4265: F, t6646: F, t828: F) -> (F, F, F, F) {
    let t87640 = t23035 * t6637 * t25319 * t2379;
    let t87642 = t81959 * t1887;
    let t87645 = t87642 * t25248 * t25249 * t2379;
    let t87650 = t1888 * t6646 * t4265 * t828 * t232;
    (t87640, t87642, t87645, t87650)
}
