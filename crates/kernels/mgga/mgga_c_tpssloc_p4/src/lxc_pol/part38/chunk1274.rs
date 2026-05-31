//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1274/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1274<F: Float>(t2371: F, t5154: F, t12134: F, t12136: F, t12138: F, t5151: F, t67: F, t758: F, t12142: F, t12127: F, t12133: F, t12141: F, t15980: F, t15983: F, t15985: F, t15987: F, t15988: F, t9853: F, t9859: F) -> (F, F, F, F, F, F, F) {
    let t16164 = t5154 * t2371;
    let t16165 = F::cast_from(0.11696447245269292414e1_f64) * t16164;
    let t16166 = F::cast_from(16.0_f64) * t12134;
    let t16167 = F::cast_from(40.0_f64) * t12136;
    let t16168 = F::cast_from(0.23392894490538584828e1_f64) * t12138;
    let t16169 = t5151 * t67;
    let t16171 = F::cast_from(0.36622894612013090108e-3_f64) * t16169 * t758;
    let t16172 = F::cast_from(0.11696447245269292414e1_f64) * t12142;
    let t16173 = t15980 + t15983 + t15985 - t15987 + t12127 + t15988 + t12133 + t16165 - t16166 + t16167 + t9853 + t16168 - t16171 + t9859 - t12141 - t16172;
    (t16165, t16166, t16167, t16168, t16171, t16172, t16173)
}
