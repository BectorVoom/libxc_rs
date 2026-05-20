//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2015/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2015<F: Float>(t1824: F, t7918: F, t1332: F, t1352: F, t19735: F, t19805: F, t2089: F, t27074: F, t29327: F, t5250: F, t5287: F, t5334: F, t5344: F, t90868: F, t90876: F, t93524: F, t93528: F, t93529: F, t93537: F, t96962: F, t96967: F, t96972: F, t96976: F, t96979: F) -> F {
    let t102562 = t7918 * t1824;
    let t102580 = -F::cast_from(0.19739208802178717238e0_f64) * t96962 + t19805 * t2089 + t1332 * t29327 - t93524 + F::new(4.0) * t5334 * t102562 * t5250 - F::new(2.0) * t5344 * t27074 * t5287 + t93528 + t93529 + F::new(4.0) * t5334 * t27074 * t19735 - t93537 + F::cast_from(0.25587863262083522345e0_f64) * t90868 + F::cast_from(0.6579736267392905746e-1_f64) * t96967 - F::new(2.0) * t5344 * t102562 * t1352 + F::cast_from(0.3289868133696452873e-1_f64) * t96972 + F::cast_from(0.3289868133696452873e-1_f64) * t96976 - F::cast_from(0.3289868133696452873e-1_f64) * t96979 + t90876;
    t102580
}
