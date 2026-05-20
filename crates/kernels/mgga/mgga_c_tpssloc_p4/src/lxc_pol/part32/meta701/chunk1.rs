//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2198/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2198<F: Float>(t1874: F, t96683: F, t25992: F, t7685: F, t25985: F, t28821: F, t7000: F, t1983: F, t24990: F, t26167: F, t7687: F, t91620: F) -> (F, F, F, F, F, F) {
    let t97831 = F::new(4.0) * t96683 * t1874;
    let t97833 = F::new(2.0) * t7685 * t25992;
    let t97835 = F::new(6.0) * t7685 * t25985;
    let t97836 = t28821 * t7000;
    let t97839 = F::new(6.0) * t1983 * t26167 * t24990;
    let t97842 = F::new(6.0) * t1983 * t91620 * t7687;
    (t97831, t97833, t97835, t97836, t97839, t97842)
}
