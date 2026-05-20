//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 881/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk881<F: Float>(t31349: F, t10110: F, t865: F, t8733: F, t31406: F, t31425: F, t2713: F, t31330: F, t31335: F, t31340: F, t31368: F, t31371: F, t31421: F, t7087: F, t7092: F, t7107: F, t855: F, t8741: F) -> (F, F, F, F, F) {
    let t32014 = F::cast_from(0.76763589786250567037e-1_f64) * t31349;
    let t32018 = t10110 * t8733 * t865;
    let t32023 = F::cast_from(0.16449340668482264365e-1_f64) * t31406;
    let t32027 = F::cast_from(0.76763589786250567037e-1_f64) * t31425;
    let t32028 = -t2713 * t8741 - F::cast_from(0.3289868133696452873e-1_f64) * t31330 + F::cast_from(0.6579736267392905746e-1_f64) * t31335 + F::cast_from(0.6579736267392905746e-1_f64) * t31340 - t32014 - F::cast_from(0.6579736267392905746e-1_f64) * t31368 - F::cast_from(0.3289868133696452873e-1_f64) * t31371 - F::new(6.0) * t855 * t32018 - F::new(2.0) * t7087 * t7107 + t32023 - F::cast_from(0.3289868133696452873e-1_f64) * t31421 + F::new(4.0) * t7087 * t7092 + t32027;
    (t32014, t32018, t32023, t32027, t32028)
}
