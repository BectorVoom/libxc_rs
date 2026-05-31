//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1192/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1192<F: Float>(t1926: F, t3158: F, t40: F, t6722: F, t1937: F, t1929: F, t34: F, t1932: F, t1934: F, t6729: F, t131: F, t23322: F) -> (F, F, F, F, F, F, F) {
    let t23447 = t1926 * t3158 / F::cast_from(432.0_f64);
    let t23448 = t6722 * t40;
    let t23449 = t23448 * t1937;
    let t23451 = t1929 * t34;
    let t23452 = F::cast_from(1.0_f64) / t23451;
    let t23453 = t23452 * t1932;
    let t23454 = t23453 * t1934;
    let t23457 = t6722 * t6729;
    let t23460 = t23322 * t131;
    (t23447, t23449, t23452, t23453, t23454, t23457, t23460)
}
