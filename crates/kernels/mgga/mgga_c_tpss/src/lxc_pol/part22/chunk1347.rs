//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1347/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1347<F: Float>(t63973: F, t63977: F, t63990: F, t61081: F, t61087: F, t61089: F, t63971: F, t63975: F, t63979: F, t63981: F, t63984: F, t63987: F, t63995: F) -> F {
    let t66427 = F::new(7.0) / F::new(576.0) * t63973;
    let t66429 = F::new(35.0) / F::new(144.0) * t63977;
    let t66434 = F::new(7.0) / F::new(12.0) * t63990;
    let t66439 = -F::new(5.0) / F::new(32.0) * t63971 + t66427 - t63975 / F::new(768.0) - t66429 + F::new(5.0) / F::new(96.0) * t63979 + F::new(5.0) / F::new(192.0) * t63981 + t63984 / F::new(4.0) + t63987 / F::new(8.0) - t66434 - t63995 / F::new(2.0) + F::new(7.0) / F::new(288.0) * t61081 - F::new(119.0) / F::new(432.0) * t61087 - F::new(35.0) / F::new(288.0) * t61089;
    t66439
}
