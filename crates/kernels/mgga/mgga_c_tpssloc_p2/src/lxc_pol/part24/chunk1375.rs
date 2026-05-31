//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1375/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1375<F: Float>(t10348: F, t1058: F, t1060: F, t11065: F, t11066: F, t1949: F, t23327: F, t23346: F, t23613: F, t23647: F, t23685: F, t23686: F, t23714: F, t23715: F, t2776: F, t3010: F, t3120: F, t6687: F, t6768: F, t6784: F, t6805: F, t82714: F, t82717: F, t82730: F, t82734: F, t82737: F, t82739: F) -> F {
    let t82749 = F::cast_from(0.43864908449286038307e-1_f64) * t23346 * t23715 - F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t23613 * t23686 - F::cast_from(0.43864908449286038307e-1_f64) * t82714 - F::cast_from(0.54831135561607547884e-2_f64) * t82717 + F::cast_from(0.16449340668482264365e-1_f64) * t23327 * t23613 * t23714 - F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t3010 * t6805 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t10348 * t1949 + F::cast_from(0.13159472534785811492e0_f64) * t23346 * t23647 - F::cast_from(6.0_f64) * t11065 * t82730 * t11066 + F::cast_from(0.82246703342411321826e-2_f64) * t82734 + F::cast_from(0.16449340668482264365e-1_f64) * t82737 - F::cast_from(0.82246703342411321826e-2_f64) * t82739 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t6784 * t23685 * t2776 + F::cast_from(3.0_f64) * t1058 * t6768 * t3120 * t1060;
    t82749
}
