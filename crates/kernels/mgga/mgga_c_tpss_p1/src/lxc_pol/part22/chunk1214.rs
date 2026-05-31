//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1214/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1214<F: Float>(t12679: F, t18690: F, t5757: F, t5936: F, t1830: F, t2105: F, t1163: F, t1273: F, t13235: F, t1760: F, t1800: F, t1834: F, t18547: F, t18613: F, t18628: F, t18680: F, t18687: F, t2056: F, t2062: F, t2065: F, t3396: F, t3499: F, t485: F, t5706: F, t5799: F, t5801: F, t5809: F, t5816: F, t5820: F, t5905: F, t5910: F, t5939: F, t626: F) -> (F, F, F, F) {
    let t18691 = t18690 * t12679;
    let t18694 = t5936 * t5757;
    let t18697 = t1830 * t2105;
    let t18704 = -F::cast_from(2.0_f64) * t1163 * t5799 + F::cast_from(2.0_f64) * t1273 * t5905 - F::cast_from(2.0_f64) * t13235 * t1800 + F::cast_from(6.0_f64) * t1760 * t18687 - F::cast_from(2.0_f64) * t1760 * t18694 - F::cast_from(2.0_f64) * t1830 * t2062 + t1834 * t3396 - F::cast_from(6.0_f64) * t18547 * t18691 - F::cast_from(2.0_f64) * t18613 * t626 - F::cast_from(2.0_f64) * t18628 * t626 - t18680 * t485 - F::cast_from(2.0_f64) * t18697 * t626 - F::cast_from(4.0_f64) * t2056 * t5816 - F::cast_from(4.0_f64) * t2056 * t5820 - F::cast_from(4.0_f64) * t2065 * t5801 - F::cast_from(4.0_f64) * t3499 * t5809 - F::cast_from(4.0_f64) * t3499 * t5816 + F::cast_from(6.0_f64) * t5706 * t5910 - F::cast_from(2.0_f64) * t5706 * t5939;
    (t18691, t18694, t18697, t18704)
}
