//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1855/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1855<F: Float>(t1022: F, t6768: F, t1060: F, t6733: F, t6743: F, t6801: F, t1945: F, t3040: F, t3201: F, t1058: F, t1920: F, t1950: F, t23323: F, t23327: F, t23601: F, t23606: F, t23610: F, t23614: F, t23619: F, t23621: F, t23626: F, t23629: F, t23633: F, t23637: F, t23642: F, t23644: F, t23647: F, t23650: F, t3180: F, t3200: F, t6687: F, t6797: F, t6811: F) -> (F, F, F, F, F, F) {
    let t23653 = t6768 * t1022;
    let t23654 = t23653 * t1060;
    let t23657 = t6733 * t6743;
    let t23658 = t23657 * t6801;
    let t23661 = t1945 * t3040;
    let t23662 = t23661 * t3201;
    let t23664 = -F::cast_from(0.82246703342411321825e-2_f64) * t23601 * t23606 + F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t23610 - F::cast_from(0.54831135561607547884e-2_f64) * t23327 * t23614 - t23619 + F::cast_from(0.82246703342411321825e-2_f64) * t1920 * t23621 + F::cast_from(0.80418998823691070228e-1_f64) * t23323 * t1950 - F::cast_from(0.14621636149762012769e-1_f64) * t23626 + F::cast_from(0.54831135561607547884e-2_f64) * t23629 + F::cast_from(0.54831135561607547884e-2_f64) * t23633 * t23637 + F::new(2.0) * t3180 * t6811 - F::cast_from(0.54831135561607547884e-2_f64) * t23642 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t23644 - F::cast_from(0.16449340668482264365e-1_f64) * t6687 * t23647 - F::cast_from(0.82246703342411321825e-2_f64) * t6687 * t23650 + F::new(2.0) * t1058 * t23654 - F::cast_from(0.16449340668482264365e-1_f64) * t6797 * t23658 - t3200 * t23662;
    (t23654, t23657, t23658, t23661, t23662, t23664)
}
