//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2548/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2548<F: Float>(t423: F, t51570: F, t51590: F, t1128: F, t15204: F, t3356: F, t4794: F, t11349: F, t1675: F, t14829: F, t3403: F, t11297: F, t11345: F, t11353: F, t1138: F, t11434: F, t1155: F, t15126: F, t15141: F, t15179: F, t15182: F, t15185: F, t1683: F, t3352: F, t3360: F, t3401: F, t44202: F, t44205: F, t44295: F, t44300: F, t4797: F, t4824: F, t4840: F, t51549: F) -> (F, F) {
    let t51593 = F::new(0.621814e-1) * (t51570 + t51590) * t423;
    let t51594 = t15204 * t1128;
    let t51599 = t4794 * t3356;
    let t51604 = t1675 * t11349;
    let t51613 = t14829 * t3403;
    let t51617 = -F::cast_from(0.35089341735807877242e1_f64) * t44202 * t4840 - F::cast_from(0.70178683471615754484e1_f64) * t11297 * t15179 - F::cast_from(0.35089341735807877242e1_f64) * t11297 * t15182 - F::cast_from(0.31168546390226634765e3_f64) * t44205 * t15185 + t51549 + t51593 + F::new(3.0) * t51594 * t1138 + F::new(3.0) * t15141 * t3352 + F::cast_from(0.96491876992155210402e2_f64) * t51599 * t3360 + F::new(1.0) * t4797 * t11345 + F::cast_from(0.2069040516770936012e4_f64) * t51604 * t11353 + F::new(1.0) * t44295 * t1683 + F::cast_from(0.96491876992155210402e2_f64) * t44300 * t4824 + F::cast_from(0.51947577317044391277e2_f64) * t15126 * t11434 + F::cast_from(0.51947577317044391277e2_f64) * t3401 * t51613 * t1155;
    (t51593, t51617)
}
