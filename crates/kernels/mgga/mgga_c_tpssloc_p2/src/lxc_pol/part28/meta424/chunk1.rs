//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1601/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1601<F: Float>(t13487: F, t22960: F, t606: F, t776: F, t25: F, t2553: F, t1887: F, t6581: F) -> (F, F, F, F) {
    let t22961 = t22960 * t13487;
    let t22964 = t606 * t776;
    let t22968 = t25 * t2553;
    let t22986 = t6581 * t1887;
    (t22961, t22964, t22968, t22986)
}
