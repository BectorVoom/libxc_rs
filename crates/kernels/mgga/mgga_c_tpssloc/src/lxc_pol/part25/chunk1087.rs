//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1087/1094 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1087<F: Float>(t85166: F, t870: F, t1877: F, t2057: F, t2058: F, t22961: F, t22968: F, t23296: F, t23299: F, t23302: F, t24191: F, t24335: F, t24339: F, t25: F, t2522: F, t26563: F, t606: F, t7110: F, t7114: F, t81470: F, t81476: F, t81486: F, t81509: F, t81513: F, t81548: F, t82320: F, t82330: F, t84797: F, t84800: F) -> (F, F) {
    let t85167 = t85166 * t870;
    let t85187 = -9.0 * t84797 * t22961 + 3.0 * t1877 * t84800 * t23296 - 9.0 * t24191 * t81548 + 3.0 / 2.0 * t2522 * t2057 * t81509 + 3.0 * t82320 * t2058 - 9.0 * t26563 * t81486 - 3.0 / 2.0 * t1877 * t7114 * t81513 + 9.0 * t26563 * t81470 + t1877 * t85167 * t25 / 2.0 + 9.0 * t24191 * t81476 + 3.0 / 2.0 * t1877 * t24335 * t606 + 9.0 / 2.0 * t2522 * t7110 * t22968 - 3.0 / 2.0 * t1877 * t24339 * t23302 - 9.0 / 2.0 * t24191 * t82330 - 3.0 * t1877 * t24339 * t23299;
    (t85167, t85187)
}
