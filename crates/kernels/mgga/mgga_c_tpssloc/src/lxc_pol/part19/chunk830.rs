//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 830/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk830<F: Float>(t10186: F, t10192: F, t10196: F, t10200: F, t10204: F, t10209: F, t10219: F, t10226: F, t10229: F, t10233: F, t10238: F, t10242: F, t10246: F, t10251: F, t10256: F, t10260: F, t10263: F, t2960: F, t2982: F, t2986: F, t2991: F, t973: F, t980: F) -> (F,) {
    let t10266 = 0.44444444444444444443e-2 * t10186 * t2991 - 0.55555555555555555554e-3 * t10192 + 0.11111111111111111111e-2 * t2986 * t10196 + 0.16666666666666666666e-2 * t973 * t10200 + 0.27777777777777777777e-3 * t973 * t10204 - 0.24999999999999999999e-2 * t973 * t10209 + 0.86419753086419753084e-3 * t973 * t10219 - 0.29629629629629629629e-2 * t2960 * t2982 - 0.18518518518518518518e-3 * t10226 + 0.27777777777777777777e-3 * t10229 + 0.37037037037037037036e-3 * t10233 - 0.11111111111111111111e-2 * t2986 * t10238 - 0.83333333333333333331e-3 * t2986 * t10242 - 0.83333333333333333331e-3 * t2986 * t10246 - 0.16666666666666666666e-2 * t2986 * t10251 + 0.16666666666666666666e-2 * t2986 * t10256 - 0.83333333333333333331e-3 * t2986 * t10260 + 0.81481481481481481478e-2 * t10263 * t980;
    (t10266,)
}
