//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1104/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1104<F: Float>(t76228: F, t76232: F, t321: F, t333: F, t4669: F, t5155: F, t69183: F, t77930: F, t77933: F, t77935: F, t77938: F, t77940: F, t77942: F, t77943: F, t77946: F, t77949: F, t80429: F) -> F {
    let t80434 = F::new(0.82834157616596963771e-1) * t76228;
    let t80435 = F::new(0.16566831523319392754e-1) * t76232;
    let t80442 = -t69183 + t77930 + t77933 - t77935 + t77938 - t77940 - t77942 + t77943 + t77946 - t80434 - t80435 - F::new(0.17961362552795712846e0) * t4669 * t80429 * t321 + F::new(0.23948483403727617128e0) * t5155 * t80429 * t333 - t77949;
    t80442
}
