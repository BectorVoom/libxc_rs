//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 904/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk904<F: Float>(t191: F, t192: F, t8107: F, t3701: F, t7939: F, t33199: F, t33208: F, t33213: F, t33216: F, t33218: F, t33224: F, t33227: F, t33230: F, t33233: F, t33236: F, t33238: F, t33239: F, t33337: F) -> (F, F, F) {
    let t33746 = t8107 * t191 * t192;
    let t33899 = t3701 * t7939;
    let t34104 = -t33199 - t33208 - t33213 - t33216 - t33218 + t33224 - t33227 - t33230 - t33233 - t33236 - t33238 + t33239 + t33337;
    (t33746, t33899, t34104)
}
