//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2487/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2487<F: Float>(t1036: F, t21483: F, t1041: F, t13969: F, t21511: F, t10413: F, t10422: F, t21531: F, t10408: F, t10937: F, t13995: F, t14511: F, t17718: F, t18021: F, t21396: F, t21520: F, t21595: F, t3070: F, t3071: F, t43361: F, t48607: F, t50148: F, t50170: F, t62602: F, t69657: F, t884: F) -> (F, F) {
    let t70766 = t21483 * t1036;
    let t70792 = t1041 * t13969 * t21511;
    let t70800 = t10413 * t10422 * t21531;
    let t70802 = t10937 * t21520 / F::new(144.0) + t3070 * t3071 * t21595 * t884 / F::new(4608.0) + t13995 * t18021 / F::new(1536.0) - t14511 * t17718 / F::new(1024.0) - t50148 - F::new(5.0) / F::new(768.0) * t48607 * t10408 * t69657 + F::new(5.0) / F::new(6912.0) * t70792 - t50170 + t62602 / F::new(1152.0) - t43361 * t3071 * t21396 * t884 / F::new(768.0) - t70800 / F::new(2304.0);
    (t70766, t70802)
}
