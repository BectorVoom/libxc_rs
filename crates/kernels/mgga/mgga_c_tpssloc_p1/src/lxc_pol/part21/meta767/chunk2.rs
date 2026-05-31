//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2647/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2647<F: Float>(t19530: F, t626: F, t12774: F, t12795: F, t12802: F, t1447: F, t16: F, t19488: F, t19489: F, t19492: F, t19499: F, t19503: F, t19504: F, t19517: F, t2219: F, t2248: F, t2336: F, t2341: F, t2351: F, t2355: F, t30171: F, t30307: F, t45697: F, t45707: F, t45751: F, t45762: F, t5469: F, t5472: F, t5475: F, t657: F, t659: F, t92: F) -> (F, F) {
    let t55420 = t626 * t19530;
    let t55457 = -F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t657 * t19504 + F::cast_from(200.0_f64) / F::cast_from(27.0_f64) * t5475 * t2355 + F::cast_from(400.0_f64) / F::cast_from(81.0_f64) * t2336 * t5469 + F::cast_from(200.0_f64) / F::cast_from(27.0_f64) * t2336 * t5472 + F::cast_from(400.0_f64) / F::cast_from(81.0_f64) * t5475 * t2351 + F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t1447 * t12802 + F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t45707 * t30307 * t2219 - F::cast_from(40.0_f64) / F::cast_from(27.0_f64) * t45697 * t30171 * t2219 - F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t12774 * t19492 * t16 + F::cast_from(20.0_f64) / F::cast_from(3.0_f64) * t12795 * t19517 * t16 - t45751 + t45762 + F::cast_from(100.0_f64) / F::cast_from(81.0_f64) * t657 * t19489 - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t92 * t19488 * t2248 - F::cast_from(100.0_f64) / F::cast_from(27.0_f64) * t657 * t19499 + F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t92 * t2341 * t19503 * t659;
    (t55420, t55457)
}
