//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2236/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2236<F: Float>(t362: F, t4657: F, t1598: F, t974: F, t23631: F, t1920: F, t25535: F, t968: F, t1003: F, t1049: F, t1058: F, t1060: F, t11059: F, t14577: F, t23633: F, t23658: F, t25429: F, t25510: F, t25550: F, t25553: F, t25706: F, t25723: F, t2770: F, t2771: F, t2780: F, t3120: F, t3961: F, t6687: F, t6784: F, t6800: F, t7593: F, t7619: F, t82668: F, t82714: F, t82717: F, t83239: F, t88016: F, t884: F) -> F {
    let t89235 = t362 * t4657;
    let t89242 = t974 * t1598;
    let t89243 = t23631 * t89242;
    let t89256 = F::cast_from(0.54831135561607547884e-2_f64) * t1920 * t968 * t25535;
    let t89265 = F::cast_from(0.73108180748810063846e-2_f64) * t25429 * t25510 * t1049 * t2770 * t3961 - F::cast_from(0.14621636149762012769e-1_f64) * t82714 - F::cast_from(0.36554090374405031922e-2_f64) * t82717 - F::cast_from(0.19495514866349350359e-1_f64) * t88016 * t25723 + F::cast_from(0.54831135561607547884e-2_f64) * t6687 * t6784 * t89235 * t884 - F::cast_from(0.14621636149762012769e-1_f64) * t82668 * t25550 - F::cast_from(0.16449340668482264365e-1_f64) * t89243 * t23658 + F::new(6.0) * t11059 * t7619 * t14577 + t1058 * t7593 * t3120 * t1060 + F::new(2.0) * t1003 * t25706 + t89256 + F::cast_from(0.27415567780803773942e-2_f64) * t23633 * t25553 * t6800 * t2780 + F::cast_from(0.36554090374405031923e-2_f64) * t83239 * t25553 * t6800 * t2771;
    t89265
}
