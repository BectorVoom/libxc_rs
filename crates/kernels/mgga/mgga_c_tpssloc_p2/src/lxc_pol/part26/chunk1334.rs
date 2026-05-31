//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1334/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1334<F: Float>(t12739: F, t6534: F, t22479: F, t5113: F, t1401: F, t81455: F, t12521: F, t3938: F, t1873: F, t3941: F, t9416: F, t16535: F) -> (F, F, F, F, F, F, F) {
    let t83966 = F::cast_from(6.0_f64) * t12739 * t6534;
    let t83968 = F::cast_from(6.0_f64) * t5113 * t22479;
    let t83979 = F::cast_from(0.135e2_f64) * t1401 * t81455;
    let t83984 = F::cast_from(0.405e2_f64) * t12521 * t6534;
    let t83988 = F::cast_from(0.405e2_f64) * t3938 * t22479;
    let t83991 = F::cast_from(27.0_f64) * t3941 * t1873 * t9416;
    let t83993 = F::cast_from(81.0_f64) * t16535 * t6534;
    (t83966, t83968, t83979, t83984, t83988, t83991, t83993)
}
