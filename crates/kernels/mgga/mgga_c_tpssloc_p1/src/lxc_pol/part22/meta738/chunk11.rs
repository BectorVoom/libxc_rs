//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2433/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2433<F: Float>(t10828: F, t14263: F, t14337: F, t17454: F, t17493: F, t17496: F, t17500: F, t21239: F, t2905: F, t2930: F, t4454: F, t4471: F, t4476: F, t49104: F, t5794: F, t60343: F, t60424: F, t69253: F, t69255: F, t69257: F, t69259: F, t69261: F, t69263: F, t69276: F, t950: F) -> F {
    let t69286 = t69253 - t69255 + t69257 - t69259 - t69261 - t69263 + F::cast_from(0.51947577317044391276e2_f64) * t14337 * t17493 + F::cast_from(0.10389515463408878255e3_f64) * t14337 * t17496 + F::cast_from(0.30762056574649219972e4_f64) * t49104 * t17500 - F::cast_from(0.31168546390226634765e3_f64) * t10828 * t5794 * t4471 - F::cast_from(0.11696447245269292414e1_f64) * t2905 * t21239 * t950 + F::cast_from(0.17315859105681463759e2_f64) * t2930 * t69276 * t950 - F::cast_from(0.35089341735807877242e1_f64) * t60424 * t4454 + F::cast_from(0.51947577317044391276e2_f64) * t60343 * t4476 - F::cast_from(0.35089341735807877242e1_f64) * t14263 * t17454;
    t69286
}
