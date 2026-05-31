//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3010/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3010<F: Float>(t3040: F, t5914: F, t3166: F, t5872: F, t1023: F, t11034: F, t11054: F, t11059: F, t14596: F, t14651: F, t18080: F, t18083: F, t18088: F, t18094: F, t18099: F, t18104: F, t18111: F, t18161: F, t3186: F, t3188: F, t3200: F, t3201: F, t43470: F, t43562: F, t4649: F, t4669: F, t4673: F, t4689: F, t50509: F, t50610: F, t5932: F) -> (F, F, F) {
    let t62925 = t5914 * t3040;
    let t62945 = t3166 * t5872;
    let t62953 = F::cast_from(4.0_f64) * t1023 * t4649 * t50509 * t50610 + F::cast_from(4.0_f64) * t11054 * t3186 * t5932 + F::cast_from(12.0_f64) * t11059 * t18080 * t18111 + F::cast_from(8.0_f64) * t18088 * t3186 * t4673 + F::cast_from(4.0_f64) * t18099 * t3186 * t4673 + F::cast_from(4.0_f64) * t18161 * t3186 * t4673 + F::cast_from(2.0_f64) * t3186 * t3188 * t62925 - t3200 * t3201 * t62945 + F::cast_from(4.0_f64) * t11034 * t18083 + F::cast_from(2.0_f64) * t14596 * t4669 + F::cast_from(4.0_f64) * t14651 * t4689 + F::cast_from(2.0_f64) * t18094 * t43562 - F::cast_from(12.0_f64) * t18104 * t43470;
    (t62925, t62945, t62953)
}
