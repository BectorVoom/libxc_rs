//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1346/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1346<F: Float>(t1945: F, t21390: F, t5872: F, t7593: F, t1615: F, t5392: F, t6800: F, t100163: F, t100240: F, t1058: F, t1060: F, t11046: F, t11048: F, t11065: F, t11066: F, t1539: F, t1599: F, t18086: F, t1948: F, t23601: F, t23604: F, t23633: F, t25516: F, t28666: F, t3186: F, t3188: F, t381: F, t5681: F, t5836: F, t5866: F, t6687: F, t6784: F, t7620: F, t83233: F, t83245: F, t89044: F) -> (F, F, F, F) {
    let t106045 = t1945 * t21390;
    let t106058 = t7593 * t5872;
    let t106073 = t5392 * t1615 * t6800;
    let t106083 = -F::cast_from(6.0_f64) * t11065 * t106045 * t11066 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t6784 * t25516 * t5681 - F::cast_from(0.24674011002723396548e-1_f64) * t6687 * t1599 * t1948 * t381 * t5836 + F::cast_from(6.0_f64) * t3186 * t106058 * t3188 + F::cast_from(3.0_f64) * t1058 * t7593 * t5866 * t1060 + t11046 * t106045 * t11048 - F::cast_from(0.49348022005446793095e-1_f64) * t23601 * t89044 * t28666 - F::cast_from(0.16449340668482264365e-1_f64) * t100163 - F::cast_from(0.16449340668482264365e-1_f64) * t23633 * t83233 * t106073 - F::cast_from(0.82246703342411321826e-2_f64) * t83245 * t100240 * t23604 * t1539 + F::cast_from(3.0_f64) * t18086 * t7620;
    (t106045, t106058, t106073, t106083)
}
