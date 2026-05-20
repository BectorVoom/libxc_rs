//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1205/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1205<F: Float>(t1030: F, t1940: F, t354: F, t1036: F, t1942: F, t1039: F) -> (F, F, F, F) {
    let t6758 = t1940 * t1030;
    let t6759 = t354 * t6758;
    let t6763 = t1942 * t1036 / F::new(2304.0);
    let t6764 = t1940 * t1039;
    (t6758, t6759, t6763, t6764)
}
