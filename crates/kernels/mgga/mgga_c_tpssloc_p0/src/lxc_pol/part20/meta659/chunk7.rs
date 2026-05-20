//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2459/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2459<F: Float>(t2770: F, t2987: F, t10277: F, t4509: F, t10390: F, t13765: F, t10937: F, t14501: F, t10408: F, t10915: F, t13554: F, t14033: F, t14037: F, t2986: F, t3070: F, t42496: F, t43303: F, t43307: F, t43310: F, t43313: F, t4575: F, t45971: F, t4644: F, t49976: F) -> F {
    let t50366 = t2987 * t2770;
    let t50370 = t4509 * t10277;
    let t50378 = t10390 * t13765;
    let t50384 = t10937 * t14501;
    let t50393 = t2986 * t50366 * t45971 / F::new(16.0) - t2986 * t50370 * t45971 / F::new(12.0) + F::new(19.0) / F::new(864.0) * t43303 - t43307 - F::new(77.0) / F::new(486.0) * t43310 - t42496 * t4575 / F::new(144.0) + t50378 / F::new(1152.0) + t10390 * t14033 / F::new(1536.0) + F::new(5.0) / F::new(4608.0) * t10390 * t14037 - t50384 / F::new(216.0) - t4644 * t10915 / F::new(768.0) - t43313 / F::new(108.0) - F::new(5.0) / F::new(768.0) * t3070 * t10408 * t13554 * t49976;
    t50393
}
