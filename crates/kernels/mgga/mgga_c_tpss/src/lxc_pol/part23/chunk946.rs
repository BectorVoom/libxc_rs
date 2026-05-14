//! MGGA_C_TPSS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 946/1354 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part23_v4rho3sigma_5_chunk946<F: Float>(t732: F, t9968: F, t1190: F, t8124: F, t1173: F, t3280: F, t3267: F, t3329: F, t509: F, t526: F, t235: F, t72: F, t3342: F, t3350: F, t1242: F, t2376: F, t339: F) -> (F, F, F, F, F, F, F) {
    let t9969 = t9968 * t732;
    let t9972 = 0.56968947174242584612e-3 * t1190 * t8124;
    let t9980 = 12.0 * t1173 * t3280;
    let t9981 = t3267 * t3329;
    let t9984 = 1.0 / t526 / t509;
    let t9986 = t235 * t9984 * t72;
    let t9991 = t3342 * t3350;
    let t9994 = t339 * t1242 * t2376;
    (t9969, t9972, t9980, t9981, t9986, t9991, t9994)
}
