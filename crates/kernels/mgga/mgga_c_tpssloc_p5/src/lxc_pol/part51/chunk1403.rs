//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1403/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1403<F: Float>(t114601: F, t1527: F, t1888: F, t23270: F, t118892: F, t118894: F, t118901: F, t118904: F, t121607: F, t13065: F, t13463: F, t2054: F, t218: F, t25188: F, t25200: F, t259: F, t2713: F, t2718: F, t31416: F, t33452: F, t6662: F, t7087: F, t7092: F, t7841: F, t855: F, t8553: F, t8563: F, t87758: F, t98975: F) -> F {
    let t121689 = t1888 * t23270 * t114601 * t1527;
    let t121691 = t218 * t121607 * t259 + t118892 - t118894 + F::new(2.0) * t25188 * t7092 - t87758 * t2054 - t13463 * t8563 + F::new(2.0) * t7087 * t25200 + F::new(2.0) * t13065 * t8553 + F::new(2.0) * t855 * t2718 * t7841 * t6662 + F::new(2.0) * t2713 * t33452 - F::new(6.0) * t98975 * t31416 - t118901 + t118904 + F::cast_from(0.16449340668482264365e-1_f64) * t121689;
    t121691
}
