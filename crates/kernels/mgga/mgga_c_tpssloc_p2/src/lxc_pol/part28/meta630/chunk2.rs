//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1975/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1975<F: Float>(t87140: F, t87153: F, t87155: F, t2627: F, t7823: F, t24273: F, t2633: F, t26654: F, t26661: F, t2679: F, t4166: F, t7837: F, t808: F, t812: F, t81595: F, t81600: F, t81602: F, t84851: F, t87117: F, t87124: F, t87133: F, t87150: F, t87159: F, t9612: F) -> F {
    let t92513 = F::cast_from(0.3289868133696452873e-1_f64) * t87140;
    let t92515 = F::cast_from(0.16449340668482264365e-1_f64) * t87153;
    let t92516 = F::cast_from(0.52089578783527170489e-1_f64) * t87155;
    let t92521 = t2627 * t7823;
    let t92528 = F::cast_from(0.6579736267392905746e-1_f64) * t87117 - F::cast_from(0.3289868133696452873e-1_f64) * t81595 - F::cast_from(0.6579736267392905746e-1_f64) * t87124 - t84851 + F::cast_from(0.10417915756705434098e0_f64) * t81600 + F::cast_from(0.25587863262083522346e0_f64) * t81602 + F::cast_from(0.6579736267392905746e-1_f64) * t87133 + t92513 + F::cast_from(0.3289868133696452873e-1_f64) * t87150 - t92515 + t92516 + F::cast_from(0.6579736267392905746e-1_f64) * t87159 - t9612 * t7837 - t812 * t26661 * t2679 + F::new(2.0) * t812 * t92521 * t2633 + F::new(2.0) * t808 * t26654 - t4166 * t24273;
    t92528
}
