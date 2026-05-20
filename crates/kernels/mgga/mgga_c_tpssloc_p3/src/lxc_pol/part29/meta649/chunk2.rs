//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2160/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2160<F: Float>(t23270: F, t25038: F, t25039: F, t2553: F, t25040: F, t82074: F, t87712: F, t82294: F, t25193: F, t81591: F, t10049: F, t13053: F, t6632: F, t6663: F, t7538: F, t82296: F, t87915: F, t87920: F, t9593: F) -> F {
    let t87924 = t25038 * t23270 * t25039 * t2553;
    let t87927 = t87712 * t82074 * t25040;
    let t87929 = F::cast_from(0.10417915756705434098e0_f64) * t82294;
    let t87931 = t81591 * t25193;
    let t87932 = F::cast_from(0.76763589786250567036e-1_f64) * t87931;
    let t87940 = -F::cast_from(0.82246703342411321824e-2_f64) * t87915 + F::cast_from(0.9869604401089358619e-1_f64) * t87920 + F::cast_from(0.49348022005446793095e-1_f64) * t87924 - F::cast_from(0.49348022005446793096e-1_f64) * t87927 - t87929 - F::cast_from(0.11514538467937585055e0_f64) * t82296 - t87932 + F::new(4.0) * t13053 * t6632 - F::new(2.0) * t9593 * t7538 - t10049 * t7538 - F::new(2.0) * t13053 * t6663;
    t87940
}
