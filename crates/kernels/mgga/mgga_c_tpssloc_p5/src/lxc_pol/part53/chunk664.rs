//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 664/1059 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk664<F: Float>(t5: F, t6976: F, t7736: F, t1992: F, t1834: F, t1998: F, t214: F, t1985: F, t2031: F, t7445: F, t1860: F, t2032: F, t7026: F, t7034: F, t7428: F, t7432: F, t7435: F) -> (F, F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t7737 = t6976 * t7736;
    let t7738 = t1992 * t7737;
    let t7740 = t1998 * t1834;
    let t7741 = t214 * t7740;
    let t7742 = t1985 * t7741;
    let t7782 = t2031 * t7445;
    let t7786 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t7428 * t2032 / F::cast_from(3.0_f64) - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7026 * t7432 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7435 * t2032 - t7034 + t1860 * t7782 / F::cast_from(3.0_f64));
    (t7737, t7738, t7740, t7741, t7742, t7782, t7786)
}
