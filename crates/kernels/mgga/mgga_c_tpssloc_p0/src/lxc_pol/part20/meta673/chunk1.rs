//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2539/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2539<F: Float>(t3316: F, t51402: F, t11300: F, t11361: F, t11430: F, t11437: F, t11441: F, t1155: F, t15126: F, t15219: F, t15222: F, t43984: F, t44188: F, t4862: F, t51133: F, t51245: F, t51248: F, t51251: F, t51382: F, t51385: F, t51389: F, t51392: F, t51399: F, t51401: F) -> (F, F) {
    let t51404 = F::cast_from(0.48245938496077605201e2_f64) * t51402 * t3316;
    let t51411 = F::cast_from(18.0_f64) * t51382 * t11437 - t51133 - t51245 + F::cast_from(0.30762056574649219974e4_f64) * t51385 * t43984 * t1155 + t51248 + t51251 + F::cast_from(0.10526802520742363173e2_f64) * t51389 * t11430 - F::cast_from(0.57895126195293126243e3_f64) * t51392 * t11441 + F::cast_from(0.35089341735807877242e1_f64) * t15126 * t11300 - t51399 - t51401 - t51404 + F::cast_from(0.51947577317044391277e2_f64) * t44188 * t4862 + F::cast_from(0.10389515463408878255e3_f64) * t11361 * t15219 + F::cast_from(0.51947577317044391277e2_f64) * t11361 * t15222;
    (t51404, t51411)
}
