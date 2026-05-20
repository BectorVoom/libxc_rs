//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2431/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2431<F: Float>(t10740: F, t10747: F, t10771: F, t10825: F, t1569: F, t1580: F, t1581: F, t17297: F, t17349: F, t17454: F, t17493: F, t17544: F, t21309: F, t21312: F, t21321: F, t2861: F, t2905: F, t2930: F, t4434: F, t4438: F, t4472: F, t48783: F, t49263: F, t49422: F, t5758: F, t5790: F, t59895: F, t69011: F, t69018: F, t69036: F) -> F {
    let t69249 = -F::cast_from(0.35089341735807877242e1_f64) * t10747 * t21309 - F::cast_from(0.35089341735807877242e1_f64) * t2905 * t4472 * t5790 - F::cast_from(0.35089341735807877242e1_f64) * t2905 * t1581 * t17297 + F::cast_from(0.51947577317044391277e2_f64) * t10825 * t21312 + F::cast_from(0.51947577317044391277e2_f64) * t2930 * t59895 * t1580 + t69011 + F::cast_from(0.10526802520742363173e2_f64) * t48783 * t17454 - t69018 - F::cast_from(0.57895126195293126241e3_f64) * t10771 * t5758 * t4438 + F::new(18.0) * t49422 * t17544 - t69036 - F::cast_from(0.31168546390226634766e3_f64) * t49263 * t17493 - F::new(6.0) * t10740 * t21321 - F::new(6.0) * t2861 * t4434 * t5758 - F::new(6.0) * t2861 * t1569 * t17349;
    t69249
}
