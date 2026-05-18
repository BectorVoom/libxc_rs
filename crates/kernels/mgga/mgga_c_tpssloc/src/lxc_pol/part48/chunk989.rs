//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 989/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk989<F: Float>(t31631: F, t6897: F, t794: F, t113981: F, t113987: F, t113964: F, t113966: F, t113969: F, t113972: F, t113975: F, t113978: F, t113983: F, t113985: F, t113989: F, t113993: F, t113997: F) -> (F, F) {
    let t115439 = t6897 * t794 * t31631;
    let t115447 = F::new(0.13457585364713463618e-3) * t113981;
    let t115450 = F::new(7.0) / F::new(144.0) * t113987;
    let t115454 = t113964 / F::new(96.0) + F::new(0.22608743412718618878e-1) * t113966 + F::new(0.32298204875312312682e-2) * t113969 - F::new(0.16149102437656156341e-2) * t113972 - F::new(0.16149102437656156341e-2) * t113975 + F::new(0.19378922925187387609e-1) * t113978 - t115447 - t113983 / F::new(96.0) + F::new(5.0) / F::new(192.0) * t113985 + t115450 - t113989 / F::new(192.0) + F::new(0.67826230238155856632e-1) * t113993 - F::new(0.96894614625936938046e-2) * t113997;
    (t115439, t115454)
}
