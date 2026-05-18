//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1357/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1357<F: Float>(t120955: F, t1983: F, t6879: F, t33234: F, t6535: F, t23938: F, t7461: F, t26977: F, t25980: F, t7042: F, t33553: F, t650: F) -> (F, F, F, F, F, F) {
    let t120958 = F::new(3.0) * t1983 * t120955 * t6879;
    let t120962 = F::new(2.0) * t33234 * t6535;
    let t120964 = F::new(2.0) * t23938 * t7461;
    let t120966 = F::new(2.0) * t26977 * t7461;
    let t120968 = F::new(2.0) * t7042 * t25980;
    let t120973 = t650 * t33553;
    (t120958, t120962, t120964, t120966, t120968, t120973)
}
