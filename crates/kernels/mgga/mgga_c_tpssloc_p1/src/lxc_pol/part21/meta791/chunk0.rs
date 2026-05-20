//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2751/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2751<F: Float>(t40722: F, t12939: F, t16619: F, t2244: F, t46234: F, t46236: F, t40729: F, t40733: F, t2517: F, t5398: F, t707: F, t10130: F, t12935: F, t193: F, t39472: F, t39476: F, t40721: F, t40732: F, t5527: F, t5544: F) -> (F, F, F, F, F, F, F, F) {
    let t57983 = F::cast_from(0.11393789434848516922e-2_f64) * t40722;
    let t57986 = F::new(24.0) * t12939 * t16619 * t2244;
    let t57987 = F::cast_from(0.69263436422725855034e2_f64) * t46234;
    let t57988 = F::cast_from(0.46785788981077169656e1_f64) * t46236;
    let t57989 = F::new(12.0) * t40729;
    let t57990 = F::cast_from(0.70178683471615754484e1_f64) * t40733;
    let t57992 = t707 * t2517 * t5398;
    let t57993 = F::new(4.0) * t57992;
    let t57994 = F::new(6.0) * t10130 * t193 * t5527 + F::new(6.0) * t12935 * t193 * t5544 - t39472 - t39476 - t40721 - t40732 - t57983 + t57986 - t57987 + t57988 + t57989 - t57990 + t57993;
    (t57983, t57986, t57987, t57988, t57989, t57990, t57993, t57994)
}
