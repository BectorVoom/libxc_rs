//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1153/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1153<F: Float>(t1354: F, t39947: F, t12365: F, t3853: F, t12267: F, t3789: F, t3798: F, t12297: F, t12385: F, t12300: F, t3858: F, t12402: F, t12407: F, t12409: F, t12413: F, t12429: F, t1341: F, t1343: F, t3795: F, t3803: F, t3805: F, t39936: F, t39938: F, t39945: F, t820: F) -> F {
    let t39948 = t39947 * t1354;
    let t39950 = t12365 * t3853;
    let t39952 = t12267 * t3789;
    let t39955 = t12267 * t3798;
    let t39956 = t39955 * t1354;
    let t39958 = t12385 * t12297;
    let t39960 = t12300 * t3858;
    let t39970 = t39936 - t1341 * t1343 * t820 * t39938 / F::cast_from(1024.0_f64) + F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t39945 - F::cast_from(119.0_f64) / F::cast_from(1152.0_f64) * t39948 - F::cast_from(119.0_f64) / F::cast_from(2304.0_f64) * t39950 + t39952 * t3795 / F::cast_from(256.0_f64) + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t39956 - F::cast_from(7.0_f64) / F::cast_from(192.0_f64) * t39958 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t39960 + t12429 * t12409 / F::cast_from(64.0_f64) + t3803 * t3805 * t12402 * t12407 / F::cast_from(128.0_f64) - t12429 * t12413 / F::cast_from(256.0_f64);
    t39970
}
