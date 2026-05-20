//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1755/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1755<F: Float>(t3701: F, t7216: F, t31: F, t63: F, t607: F, t7939: F, t1390: F, t22811: F, t2233: F, t2239: F, t601: F, t9238: F) -> (F, F, F, F, F, F, F) {
    let t32193 = t3701 * t7216;
    let t32331 = t63 * t31;
    let t32332 = t32331 * t607;
    let t33899 = t3701 * t7939;
    let t34711 = t7216 * t1390;
    let t39041 = F::new(1.0) / t22811;
    let t39049 = t2233 * t2239;
    let t39054 = t601 * t9238;
    (t32193, t32332, t33899, t34711, t39041, t39049, t39054)
}
