//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1459/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1459<F: Float>(t103218: F, t103391: F, t104647: F, t11606: F, t1238: F, t1720: F, t19249: F, t27406: F, t27751: F, t27830: F, t29532: F, t29546: F, t29557: F, t29664: F, t29671: F, t29682: F, t29690: F, t29795: F, t29803: F, t4945: F, t498: F, t5055: F, t6150: F, t6243: F, t6268: F, t7283: F, t7999: F, t8002: F, t8006: F, t8054: F, t8061: F, t8087: F, t94754: F) -> F {
    let t109927 = -F::cast_from(0.24125699647107321069e0_f64) * t103218 * t8006 + F::cast_from(0.49348022005446793095e-1_f64) * t7283 * t27751 * t29803 - F::cast_from(0.65797362673929057459e-1_f64) * t7999 * t29671 - F::cast_from(0.82246703342411321826e-2_f64) * t7283 * t103391 * t8002 + F::cast_from(12.0_f64) * t5055 * t29532 - F::cast_from(18.0_f64) * t1238 * t11606 * t8087 * t6243 + F::cast_from(0.65797362673929057459e-1_f64) * t27406 * t29682 - F::cast_from(0.13159472534785811492e0_f64) * t27406 * t29557 + F::cast_from(0.65797362673929057459e-1_f64) * t27406 * t29546 - F::cast_from(3.0_f64) * t4945 * t29795 + F::cast_from(0.43864908449286038307e-1_f64) * t104647 - F::cast_from(3.0_f64) * t27830 * t6268 + F::cast_from(3.0_f64) * t1720 * t29664 * t498 + F::cast_from(6.0_f64) * t19249 * t8061 + F::cast_from(3.0_f64) * t6150 * t8054 * t498 + F::cast_from(0.10966227112321509577e-1_f64) * t7283 * t94754 * t29690;
    t109927
}
