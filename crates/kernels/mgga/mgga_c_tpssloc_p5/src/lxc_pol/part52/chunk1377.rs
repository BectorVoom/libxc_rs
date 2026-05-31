//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1377/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1377<F: Float>(t191: F, t192: F, t27903: F, t2020: F, t104977: F, t1874: F, t27863: F, t6525: F, t116152: F, t119867: F, t119869: F, t119871: F, t119874: F, t119875: F, t123067: F, t1459: F, t31880: F, t32676: F, t32679: F, t4037: F) -> F {
    let t123111 = t27903 * t191 * t192;
    let t123112 = t123111 * t2020;
    let t123113 = t104977 * t1874;
    let t123115 = t27863 * t6525;
    let t123117 = -F::cast_from(2.0_f64) * t116152 * t1459 - F::cast_from(2.0_f64) * t123067 * t1459 - F::cast_from(2.0_f64) * t31880 * t4037 - t119867 - F::cast_from(2.0_f64) * t119869 - F::cast_from(2.0_f64) * t119871 - t119874 + t119875 + t123112 - F::cast_from(2.0_f64) * t123113 - F::cast_from(2.0_f64) * t123115 - t32676 - t32679;
    t123117
}
