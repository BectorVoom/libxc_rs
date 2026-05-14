//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 852/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk852<F: Float>(t22986: F, t23270: F, t31332: F, t87036: F, t31338: F, t82159: F, t31329: F, t6547: F, t1880: F, t214: F, t225: F, t24234: F, t258: F, t23030: F, t31319: F, t23168: F, t31367: F) -> (F, F, F, F, F, F) {
    let t114877 = t22986 * t23270 * t31332 * t87036;
    let t114880 = t22986 * t82159 * t31338;
    let t114882 = t6547 * t31329;
    let t114889 = t1880 * t214 * t24234 * t225 * t258;
    let t114891 = t23030 * t31319;
    let t114892 = 0.26044789391763585244e-1 * t114891;
    let t114900 = t23168 * t31367;
    (t114877, t114880, t114882, t114889, t114892, t114900)
}
