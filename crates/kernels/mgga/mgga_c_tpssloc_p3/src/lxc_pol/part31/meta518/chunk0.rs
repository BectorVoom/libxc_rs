//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1720/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1720<F: Float>(t29009: F, t29054: F, t858: F, t2053: F, t2718: F, t5657: F, t218: F, t29040: F, t1528: F, t17090: F, t2054: F, t25036: F, t25049: F, t259: F, t26713: F, t28265: F, t28269: F, t28274: F, t28278: F, t28289: F, t28296: F, t28300: F, t4147: F, t4268: F, t5637: F, t7087: F, t7830: F, t855: F) -> (F, F, F, F, F) {
    let t29055 = t29009 + t29054;
    let t29056 = t858 * t29055;
    let t29060 = t2718 * t2053 * t5657;
    let t29071 = t218 * t29040;
    let t29075 = -F::cast_from(0.16449340668482264365e-1_f64) * t25036 - F::cast_from(0.16449340668482264365e-1_f64) * t28265 + F::cast_from(0.6579736267392905746e-1_f64) * t28269 - F::cast_from(0.76763589786250567036e-1_f64) * t25049 + F::cast_from(0.16449340668482264365e-1_f64) * t28274 + F::new(4.0) * t4268 * t7830 - t855 * t29056 - F::cast_from(0.3289868133696452873e-1_f64) * t28278 + F::new(2.0) * t855 * t29060 - t17090 * t2054 - F::cast_from(0.6579736267392905746e-1_f64) * t28289 + F::new(2.0) * t7087 * t5637 - F::new(2.0) * t26713 * t1528 + F::cast_from(0.3289868133696452873e-1_f64) * t28296 + F::cast_from(0.9869604401089358619e-1_f64) * t28300 + t29071 * t259 + F::new(4.0) * t4147 * t7830;
    (t29055, t29056, t29060, t29071, t29075)
}
