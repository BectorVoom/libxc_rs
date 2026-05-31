//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 1018/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk1018<F: Float>(t123570: F, t123583: F, t10110: F, t114900: F, t121629: F, t121637: F, t121648: F, t121660: F, t13042: F, t218: F, t24297: F, t24305: F, t259: F, t2597: F, t26690: F, t26703: F, t26713: F, t31999: F, t32018: F, t33951: F, t4147: F, t4268: F, t4300: F, t7087: F, t7092: F, t7842: F, t855: F, t8733: F, t8734: F) -> (F, F) {
    let t123584 = t123570 + t123583;
    let t123612 = -F::cast_from(0.76763589786250567037e-1_f64) * t121629 + t218 * t123584 * t259 + F::cast_from(4.0_f64) * t26713 * t7092 - F::cast_from(6.0_f64) * t855 * t10110 * t8733 * t4300 - F::cast_from(2.0_f64) * t24305 * t7842 - F::cast_from(6.0_f64) * t4268 * t32018 + F::cast_from(0.6579736267392905746e-1_f64) * t121637 + F::cast_from(4.0_f64) * t7087 * t26690 - F::cast_from(6.0_f64) * t2597 * t33951 + F::cast_from(0.15352717957250113407e0_f64) * t114900 + F::cast_from(2.0_f64) * t13042 * t8734 + F::cast_from(4.0_f64) * t7087 * t26703 + F::cast_from(0.6579736267392905746e-1_f64) * t121648 - F::cast_from(2.0_f64) * t24297 * t7842 - t4147 * t31999 + F::cast_from(0.76763589786250567037e-1_f64) * t121660;
    (t123584, t123612)
}
