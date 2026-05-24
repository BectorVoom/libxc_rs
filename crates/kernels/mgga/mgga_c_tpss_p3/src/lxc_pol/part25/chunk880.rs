//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 880/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk880<F: Float>(t2204: F, t2214: F, t719: F, t123: F, t173: F, t186: F, t2256: F, t2267: F, t2320: F, t2327: F, t2328: F, t262: F, t706: F, t7829: F, t7922: F, t7929: F, t7932: F, t7936: F, t7945: F, t7946: F, t7954: F, t7960: F, t7972: F, t7975: F, t7979: F, t7988: F, t7992: F) -> F {
    let t7993 = t2204 * t2214;
    let t7994 = t7993 * t719;
    let t7997 = -F::cast_from(0.48159733137676571078e0_f64) * t262 * t7922 * t2328 - t7929 + t7932 + t7936 - t7945 - F::cast_from(0.35089341735807877242e1_f64) * t2320 * t7946 + F::cast_from(0.16562821945185185185e-2_f64) * t123 * t7829 * t173 + t7954 + t7960 - t7972 - t7975 - t7979 + F::cast_from(0.56968947174242584612e-3_f64) * t123 * t7829 * t186 - F::new(6.0) * t2256 * t706 * t2267 - t7988 - t7992 + F::cast_from(0.51947577317044391277e2_f64) * t2327 * t7994;
    t7997
}
