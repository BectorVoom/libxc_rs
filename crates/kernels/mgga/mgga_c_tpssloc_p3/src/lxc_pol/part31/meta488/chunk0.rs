//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1666/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1666<F: Float>(t24175: F, t7687: F, t6999: F, t7940: F, t532: F, t7939: F, t6879: F, t12571: F, t7025: F) -> (F, F, F, F, F) {
    let t26898 = t24175 * t7687;
    let t26902 = t7940 * t6999;
    let t26905 = t532 * t7939;
    let t26906 = t26905 * t6879;
    let t26911 = t12571 * t7025;
    (t26898, t26902, t26905, t26906, t26911)
}
