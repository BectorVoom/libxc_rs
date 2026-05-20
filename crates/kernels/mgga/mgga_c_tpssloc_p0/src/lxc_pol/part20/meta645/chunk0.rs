//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2368/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2368<F: Float>(t10216: F, t13797: F, t3067: F, t353: F, t373: F, t383: F, t1021: F, t820: F, t10482: F, t1615: F, t10390: F, t10858: F, t10883: F, t13975: F, t14069: F, t14080: F, t14211: F, t2986: F, t3039: F, t3041: F, t3057: F, t3064: F, t3121: F, t42388: F, t42397: F, t42436: F, t42460: F, t42511: F, t43235: F, t43361: F, t4575: F, t4582: F, t4593: F, t45971: F, t48265: F) -> (F, F, F, F) {
    let t48585 = t13797 * t10216;
    let t48607 = t353 * t383 * t3067 * t373;
    let t48611 = t820 * t1021;
    let t48612 = t1615 * t10482;
    let t48622 = t42436 / F::new(1152.0) + t10390 * t14069 / F::new(768.0) + t42511 * t4575 / F::new(1536.0) + F::new(7.0) / F::new(216.0) * t2986 * t48585 * t45971 - t14080 * t3057 / F::new(288.0) - F::new(5.0) / F::new(864.0) * t14080 * t3064 - t3039 * t4582 * t13975 * t3121 / F::new(1024.0) + t10883 * t4582 * t13975 * t3041 / F::new(1024.0) - t3039 * t4582 * t4593 * t10858 / F::new(3072.0) + F::new(5.0) / F::new(1728.0) * t48607 * t42397 * t48265 + F::new(3.0) / F::new(512.0) * t42388 * t48611 * t48612 * t43235 - F::new(3.0) / F::new(512.0) * t43361 * t48611 * t14211 * t43235 + t42460 / F::new(54.0);
    (t48607, t48611, t48612, t48622)
}
