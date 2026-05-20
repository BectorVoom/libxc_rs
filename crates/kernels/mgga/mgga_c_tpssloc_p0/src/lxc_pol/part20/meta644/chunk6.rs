//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2364/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2364<F: Float>(t14488: F, t376: F, t1023: F, t10408: F, t1041: F, t10413: F, t14107: F, t14220: F, t14222: F, t3039: F, t3070: F, t3071: F, t42322: F, t42324: F, t42354: F, t42369: F, t42372: F, t42546: F, t43211: F, t4337: F, t4342: F, t4582: F, t4588: F, t45993: F, t48472: F, t48477: F, t48496: F, t48497: F) -> (F, F) {
    let t48506 = t376 * t14488;
    let t48511 = -t3070 * t3071 * t4342 * t48472 / F::new(768.0) - t10413 * t3071 * t48477 * t14220 / F::new(768.0) + F::new(5.0) / F::new(4608.0) * t3070 * t10408 * t4337 * t48472 - t42546 * t14222 / F::new(768.0) - t43211 * t14107 / F::new(192.0) + t42322 / F::new(6912.0) + F::new(5.0) / F::new(6912.0) * t42324 + F::new(5.0) / F::new(13824.0) * t1041 * t4582 * t4588 * t45993 + F::new(55.0) / F::new(15552.0) * t1041 * t4582 * t48496 * t48497 - t42369 / F::new(1152.0) + F::new(5.0) / F::new(6912.0) * t42372 + t42354 * t14107 / F::new(1024.0) - t3039 * t4582 * t48506 * t1023 / F::new(1024.0);
    (t48506, t48511)
}
