//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2396/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2396<F: Float>(t10750: F, t10757: F, t10820: F, t10829: F, t14337: F, t14344: F, t1581: F, t2900: F, t2907: F, t42106: F, t4472: F, t49080: F, t49082: F, t49084: F, t49086: F, t49088: F, t49090: F, t49092: F, t49095: F, t49096: F, t49099: F, t49104: F) -> F {
    let t49113 = -t49080 + t49082 - t49084 + t49086 - t49088 + t49090 - t49092 + t49095 - F::cast_from(0.35089341735807877242e1_f64) * t49096 * t2907 - F::cast_from(0.10389515463408878255e3_f64) * t49099 * t10829 + F::cast_from(0.35089341735807877242e1_f64) * t14337 * t10750 + F::cast_from(0.10254018858216406658e4_f64) * t49104 * t10757 + F::cast_from(0.5848223622634646207e0_f64) * t42106 * t1581 + F::cast_from(0.17544670867903938621e1_f64) * t10820 * t4472 + F::cast_from(0.17544670867903938621e1_f64) * t2900 * t14344;
    t49113
}
