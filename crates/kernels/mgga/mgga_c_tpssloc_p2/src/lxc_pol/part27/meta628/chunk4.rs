//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2117/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2117<F: Float>(t25365: F, t868: F, t25373: F, t58009: F, t4255: F, t22960: F, t1408: F, t1877: F, t1915: F, t2249: F, t22959: F, t22964: F, t23286: F, t23299: F, t25013: F, t25028: F, t2522: F, t25358: F, t25372: F, t47645: F, t6666: F, t7475: F, t7476: F, t7541: F, t7545: F, t81525: F, t86757: F, t86764: F, t86771: F, t86775: F) -> (F, F, F) {
    let t86781 = t25365 * t868;
    let t86782 = t25373 * t86781;
    let t86794 = t25373 * t58009;
    let t86797 = t4255 * t868;
    let t86798 = t22960 * t86797;
    let t86801 = t86757 - t1877 * t81525 * t7545 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t23286 * t7475 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t1915 * t86764 + F::cast_from(3.0_f64) * t47645 * t7476 + F::cast_from(2.0_f64) * t25372 * t86771 + t86775 + t1877 * t23286 * t1408 / F::cast_from(2.0_f64) - t1877 * t25358 * t23299 + F::cast_from(6.0_f64) * t22959 * t86782 + F::cast_from(3.0_f64) * t2522 * t7541 * t22964 + t1877 * t7541 * t2249 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) * t2522 * t6666 * t25028 + F::cast_from(2.0_f64) * t25372 * t86794 - F::cast_from(6.0_f64) * t25013 * t86798;
    (t86781, t86797, t86801)
}
