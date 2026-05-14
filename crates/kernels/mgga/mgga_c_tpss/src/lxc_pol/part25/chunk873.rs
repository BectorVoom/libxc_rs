//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 873/1265 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk873<F: Float>(t30: F, t490: F, t33: F, t493: F, t1193: F, t8115: F, t8110: F, t2222: F, t3190: F, t1186: F, t3211: F, t27: F, t558: F, t498: F, t1190: F, t8124: F) -> (F, F, F, F, F, F, F, F) {
    let t9922 = t30 * t30;
    let t9924 = 1.0 / t490 / t9922;
    let t9934 = t33 * t33;
    let t9936 = 1.0 / t493 / t9934;
    let t9954 = 0.51947577317044391277e2 * t1193 * t8115;
    let t9956 = 0.35089341735807877242e1 * t1193 * t8110;
    let t9957 = t3190 * t2222;
    let t9959 = t3211 * t1186;
    let t9965 = t558 * t27;
    let t9966 = t9965 * t498;
    let t9972 = 0.56968947174242584612e-3 * t1190 * t8124;
    (t9924, t9936, t9954, t9956, t9957, t9959, t9966, t9972)
}
