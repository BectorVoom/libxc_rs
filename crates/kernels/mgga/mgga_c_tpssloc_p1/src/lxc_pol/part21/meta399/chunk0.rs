//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1876/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1876<F: Float>(t1557: F, t2793: F, t2842: F, t4434: F, t931: F, t10740: F, t10765: F, t14376: F, t14378: F, t14381: F, t14384: F, t14387: F, t14391: F, t14394: F, t14398: F, t14419: F, t2861: F, t311: F, t4416: F, t4438: F) -> (F, F, F, F) {
    let t14422 = t1557 * t2793;
    let t14424 = F::cast_from(6.0_f64) * t2842 * t14422;
    let t14425 = t4434 * t931;
    let t14428 = t14376 - t14378 + t14381 + t14384 + t14387 - t14391 - t14394 - t14398 - F::cast_from(4.0_f64) * t10740 * t4416 + F::cast_from(0.64327917994770140268e2_f64) * t10765 * t4438 - F::cast_from(0.310907e-1_f64) * t14419 * t311 - t14424 - F::cast_from(4.0_f64) * t2861 * t14425;
    (t14422, t14424, t14425, t14428)
}
