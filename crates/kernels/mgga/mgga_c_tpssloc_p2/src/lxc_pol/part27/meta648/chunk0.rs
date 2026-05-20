//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2238/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2238<F: Float>(t7611: F, t82713: F, t82716: F, t3040: F, t7593: F, t25550: F, t82822: F, t23384: F, t25476: F, t1058: F, t1060: F, t13940: F, t14488: F, t14618: F, t1945: F, t1953: F, t23701: F, t25499: F, t25516: F, t25535: F, t2776: F, t3186: F, t3200: F, t3201: F, t4615: F, t4673: F, t6687: F, t6784: F, t6797: F, t6813: F, t7610: F, t82592: F, t986: F) -> (F, F) {
    let t89309 = F::cast_from(0.14621636149762012769e-1_f64) * t82713 * t7611;
    let t89310 = t82716 * t7611;
    let t89312 = t7593 * t3040;
    let t89327 = F::cast_from(0.18277045187202515961e-2_f64) * t82822 * t25550;
    let t89329 = F::cast_from(0.18277045187202515961e-2_f64) * t23384 * t25476;
    let t89330 = -F::cast_from(0.82246703342411321825e-2_f64) * t6797 * t82592 * t7610 + t1058 * t1945 * t14488 * t1060 - F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t6784 * t25516 * t2776 - t89309 - F::cast_from(0.18277045187202515961e-2_f64) * t89310 - t3200 * t89312 * t3201 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t986 * t25535 + t13940 * t1953 + F::new(2.0) * t4615 * t6813 + F::new(4.0) * t3186 * t25499 * t4673 + F::new(2.0) * t14618 * t23701 + t89327 + t89329;
    (t89312, t89330)
}
