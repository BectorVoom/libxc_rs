//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2200/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2200<F: Float>(t25010: F, t7685: F, t16944: F, t25014: F, t25365: F, t86721: F, t22960: F, t67128: F, t1877: F, t2219: F, t7541: F, t5527: F, t606: F) -> (F, F, F, F, F, F) {
    let t97949 = F::new(2.0) * t7685 * t25010;
    let t97950 = t25014 * t16944;
    let t97953 = t86721 * t25365;
    let t97956 = t22960 * t67128;
    let t97972 = F::new(2.0) * t1877 * t7541 * t2219;
    let t97985 = t606 * t5527;
    (t97949, t97950, t97953, t97956, t97972, t97985)
}
