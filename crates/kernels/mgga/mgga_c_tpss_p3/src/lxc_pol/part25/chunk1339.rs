//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1339/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1339<F: Float>(t1981: F, t4573: F, t68: F, t1289: F, t578: F, t1792: F, t18649: F, t21123: F, t21129: F, t21133: F, t5489: F, t5785: F, t5794: F, t69135: F, t69139: F, t69228: F, t69232: F, t69242: F, t69245: F, t69248: F, t69251: F) -> F {
    let t71447 = t1981 * t4573 * t68;
    let t71451 = t578 * t1289 * t68;
    let t71460 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t21123 * t5794 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t18649 * t21129 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t5785 * t69135 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t5785 * t69139 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t18649 * t21133 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t5785 * t69228 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t5785 * t69232 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t71447 * t5489 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t71451 * t69242 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t69245 * t1792 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t69248 * t1792 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t69251 * t1792;
    t71460
}
