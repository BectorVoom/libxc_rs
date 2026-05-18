//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 739/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk739<F: Float>(t22690: F, t6638: F, t23171: F, t828: F, t852: F, t232: F, t6646: F, t1888: F, t10097: F, t206: F, t268: F, t6559: F) -> (F, F, F, F, F) {
    let t23172 = t22690 * t6638;
    let t23173 = t23171 * t23172;
    let t23174 = F::new(0.82246703342411321824e-2) * t23173;
    let t23175 = t852 * t828;
    let t23176 = t23175 * t232;
    let t23177 = t6646 * t23176;
    let t23178 = t1888 * t23177;
    let t23180 = t10097 * t232;
    let t23181 = t6646 * t23180;
    let t23182 = t1888 * t23181;
    let t23185 = t6559 * t206 * t268;
    (t23173, t23174, t23178, t23182, t23185)
}
