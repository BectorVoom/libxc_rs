//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1191/1193 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1191<F: Float>(t1858: F, t8153: F, t2193: F, t5363: F, t30263: F, t576: F, t110020: F, t110024: F, t110032: F, t110268: F, t1396: F, t1404: F, t16546: F, t2187: F, t30218: F, t3946: F, t5364: F, t5381: F, t8154: F, t8171: F, t8241: F) -> (F,) {
    let t110899 = 2.0 * t8153 * t1858;
    let t110904 = 2.0 * t5363 * t2193;
    let t110910 = 2.0 * t576 * t30263;
    let t110911 = 2.0 * t1396 * t30263 + 2.0 * t1404 * t30218 + t16546 * t2187 + t3946 * t8241 + 2.0 * t5364 * t8171 + 2.0 * t5381 * t8154 + 2.0 * t110020 + t110024 + t110032 + t110268 + t110899 + t110904 + t110910;
    (t110911,)
}
