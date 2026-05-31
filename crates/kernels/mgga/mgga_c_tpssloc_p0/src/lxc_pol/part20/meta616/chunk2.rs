//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2226/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2226<F: Float>(t52: F, t10913: F, t12606: F, t12874: F, t12877: F, t1409: F, t2244: F, t2250: F, t2440: F, t3966: F, t40647: F, t4087: F, t45872: F, t607: F, t76: F, t9258: F, t9288: F, t9438: F, zeta_threshold: F) -> F {
    let t150 = t52 <= zeta_threshold;
    let t46190 = piecewise3::<F>(t150, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t40647 * t1409 * t9288 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t9438 * t3966 * t2244 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t12874 * t10913 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t2440 * t12606 * t607 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t12877 * t2250 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t4087 * t9258 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t76 * t45872);
    t46190
}
