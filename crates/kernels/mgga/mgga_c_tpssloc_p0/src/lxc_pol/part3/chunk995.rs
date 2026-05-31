//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 995/1255 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk995<F: Float>(t40: F, t1409: F, t9427: F, t2433: F, t3966: F, t12606: F, t2244: F, t2250: F, t4080: F, t607: F, t73: F, t9438: F, t2440: F, zeta_threshold: F) -> (F, F, F) {
    let t146 = t40 <= zeta_threshold;
    let t12862 = t9427 * t1409;
    let t12865 = t2433 * t3966;
    let t12873 = piecewise3::<F>(t146, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t12862 * t2244 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t12865 * t607 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4080 * t2250 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t73 * t12606);
    let t12874 = t9438 * t1409;
    let t12877 = t2440 * t3966;
    (t12873, t12874, t12877)
}
