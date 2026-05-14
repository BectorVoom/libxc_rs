//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 133/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk133<F: Float>(t209: F, t469: F, t6: F, t605: F, t489: F, t490: F, t589: F, t467: F, t487: F, t488: F) -> (F, F, F) {
    let t608 = t469 * t6 * t605 * t209;
    let t612 = t489 * t490 * t589;
    let t615 = -0.27439556402611977244e-1 * t467 * t608 - t487 - 0.54879112805223954488e-1 * t488 * t612;
    (t608, t612, t615)
}
