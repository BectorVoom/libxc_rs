//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2429/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2429<F: Float>(t10756: F, t10825: F, t14332: F, t14369: F, t1581: F, t17350: F, t17355: F, t21115: F, t21195: F, t21198: F, t21247: F, t2856: F, t41984: F, t42149: F, t4411: F, t4472: F, t48789: F, t49096: F, t5762: F, t5775: F, t5790: F, t5791: F, t60338: F, t68758: F, t68926: F, t68995: F, t69066: F, t69079: F, t69093: F, t69105: F, t69118: F, t69130: F, t69143: F, t69156: F, t924: F, t932: F, t950: F) -> F {
    let t69180 = F::new(3.0) * t4411 * t17350 + F::cast_from(0.96491876992155210402e2_f64) * t48789 * t5762 - t68926 - F::cast_from(0.19298375398431042081e3_f64) * t41984 * t21115 + F::new(1.0) * t2856 * t21195 + F::new(1.0) * t924 * (t69066 + t69079 + t69093 + t69105 + t69118 + t69130 + t69143 + t69156) * t932 + F::cast_from(0.2069040516770936012e4_f64) * t42149 * t21198 + F::cast_from(0.17544670867903938621e1_f64) * t60338 * t1581 + F::cast_from(0.17544670867903938621e1_f64) * t17355 * t4472 + F::cast_from(0.17544670867903938621e1_f64) * t14332 * t5791 + F::cast_from(0.30762056574649219973e4_f64) * t10756 * t5790 * t14369 * t950 - F::cast_from(0.19751673498613801407e-1_f64) * t68758 + t68995 - F::cast_from(0.35089341735807877242e1_f64) * t49096 * t5775 + F::cast_from(0.35089341735807877242e1_f64) * t10825 * t21247;
    t69180
}
