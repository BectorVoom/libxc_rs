//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1783/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1783<F: Float>(t81153: F, t81317: F, t81398: F, t531: F, t7216: F, t2056: F, t40772: F, t193: F, t201: F, t7109: F, t10143: F, t82069: F) -> (F, F, F, F, F, F, F, F) {
    let t84597 = F::cast_from(0.19739208802178717238e0_f64) * t81153;
    let t84659 = F::cast_from(0.55440370401180965083e0_f64) * t81317;
    let t84705 = F::cast_from(0.27415567780803773942e-2_f64) * t81398;
    let t84733 = t531 * t7216;
    let t84766 = t2056 * t40772;
    let t84797 = t193 * t201 * t7109;
    let t84800 = t7109 * t10143;
    let t84820 = F::cast_from(0.19739208802178717238e0_f64) * t82069;
    (t84597, t84659, t84705, t84733, t84766, t84797, t84800, t84820)
}
