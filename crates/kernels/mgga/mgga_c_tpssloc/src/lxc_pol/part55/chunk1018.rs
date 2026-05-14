//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1018/1154 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1018<F: Float>(t43705: F, t11604: F, t496: F, t1406: F, t9238: F, t2239: F, t3951: F, t12461: F, t5356: F, t111: F, t5363: F, t1851: F, t671: F, t1372: F, t794: F, t213: F, t225: F) -> (F, F, F, F, F, F, F, F, F) {
    let t43706 = 1.0 / t43705;
    let t45349 = 1.0 / t11604 / t496;
    let t45844 = t1406 * t9238;
    let t46104 = t3951 * t2239;
    let t55242 = t5356 * t12461;
    let t55353 = t5363 * t111;
    let t75795 = t1851 * t671;
    let t80645 = t794 * t1372;
    let t80650 = t213 * t1372 * t225;
    (t43706, t45349, t45844, t46104, t55242, t55353, t75795, t80645, t80650)
}
