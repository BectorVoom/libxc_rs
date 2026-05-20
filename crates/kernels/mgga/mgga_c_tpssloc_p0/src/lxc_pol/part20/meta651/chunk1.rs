//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2395/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2395<F: Float>(t10817: F, t14379: F, t10655: F, t14389: F, t13655: F, t2792: F, t912: F, t2904: F, t4446: F, t10523: F, t1573: F, t10629: F) -> (F, F, F, F, F, F) {
    let t49090 = F::new(12.0) * t10817 * t14379;
    let t49092 = F::cast_from(0.96491876992155210402e2_f64) * t10655 * t14389;
    let t49095 = F::new(6.0) * t2792 * t13655 * t912;
    let t49096 = t4446 * t2904;
    let t49099 = t1573 * t10523;
    let t49104 = t1573 * t10629;
    (t49090, t49092, t49095, t49096, t49099, t49104)
}
