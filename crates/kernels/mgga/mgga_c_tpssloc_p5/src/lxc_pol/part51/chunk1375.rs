//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1375/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1375<F: Float>(t26977: F, t7468: F, t26003: F, t7042: F, t31304: F, t7756: F, t33553: F, t652: F, t671: F, t4072: F, t8595: F, t1983: F, t27144: F, t8643: F) -> (F, F, F, F, F, F) {
    let t121231 = F::cast_from(2.0_f64) * t26977 * t7468;
    let t121233 = F::cast_from(2.0_f64) * t7042 * t26003;
    let t121234 = t31304 * t7756;
    let t121237 = F::cast_from(2.0_f64) * t652 * t33553 * t671;
    let t121240 = F::cast_from(2.0_f64) * t652 * t8595 * t4072;
    let t121253 = t1983 * t27144 * t8643;
    (t121231, t121233, t121234, t121237, t121240, t121253)
}
