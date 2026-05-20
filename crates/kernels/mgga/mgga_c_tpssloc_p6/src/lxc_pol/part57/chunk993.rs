//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 993/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk993<F: Float>(t114770: F, t22986: F, t28267: F, t28276: F, t31366: F, t6552: F, t23035: F, t31376: F, t5527: F, t6637: F, t121495: F, t1510: F, t6646: F) -> (F, F, F, F) {
    let t127952 = t22986 * t114770 * t28267;
    let t127955 = t6552 * t31366 * t28276;
    let t127959 = t23035 * t6637 * t31376 * t5527;
    let t127963 = t22986 * t6646 * t121495 * t1510;
    (t127952, t127955, t127959, t127963)
}
