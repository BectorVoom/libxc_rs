//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2207/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2207<F: Float>(t12757: F, t12808: F, t1453: F, t2331: F, t2358: F, t26129: F, t29903: F, t45424: F, t45428: F, t45430: F, t45435: F, t45676: F, t45689: F, t45690: F, t45731: F, t45775: F, t64: F, t656: F, t666: F, t9366: F) -> F {
    let t45780 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t64 * t2331 * t12808 * t666 + F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t64 * t12757 * t2358 + F::cast_from(6.0_f64) * t45676 + F::cast_from(3.0_f64) * t64 * t45435 * t1453 * t9366 - F::cast_from(9.0_f64) / F::cast_from(4.0_f64) * t29903 * t26129 * t2358 + F::cast_from(22.0_f64) / F::cast_from(3.0_f64) * t45424 + F::cast_from(2.0_f64) * t45428 - F::cast_from(2.0_f64) * t45430 - t45689 + t45690 - t64 * t656 * (t45731 + t45775) / F::cast_from(8.0_f64);
    t45780
}
