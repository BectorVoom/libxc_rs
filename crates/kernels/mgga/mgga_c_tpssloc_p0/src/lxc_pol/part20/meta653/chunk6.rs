//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2414/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2414<F: Float>(t42212: F, t42213: F, t47781: F, t47785: F, t47787: F, t49043: F, t49049: F, t49052: F, t49054: F, t49056: F, t49058: F, t49060: F) -> F {
    let t49397 = F::cast_from(0.794188125e1_f64) * t49043 + t42212 + t42213 - F::cast_from(0.17215833333333333333e1_f64) * t47781 - F::new(0.929655e1) * t47785 + F::cast_from(0.53560370370370370369e0_f64) * t47787 - F::new(0.52945875e1) * t49049 + F::new(0.94674375e0) * t49052 + F::new(0.94674375e0) * t49054 + F::new(0.31558125e0) * t49056 - F::new(0.52945875e1) * t49058 + F::new(0.3529725e1) * t49060;
    t49397
}
