//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 884/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk884<F: Float>(t1393: F, t2036: F, t2040: F, t2096: F, t2165: F, t27888: F, t31753: F, t31761: F, t31769: F, t31771: F, t31774: F, t31778: F, t31832: F, t32350: F, t672: F, t7040: F, t7050: F, t7218: F, t7266: F, t7408: F, t8690: F, t8840: F) -> F {
    let t32390 = t1393 * t8840 - t2036 * t7408 - F::new(2.0) * t2040 * t27888 + t2096 * t31832 - t2165 * t7040 - F::new(2.0) * t32350 * t672 - F::new(2.0) * t7050 * t7266 + t7218 * t8690 - t31753 + t31761 - t31769 - t31771 - t31774 + t31778;
    t32390
}
