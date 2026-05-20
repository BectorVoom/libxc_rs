//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2451/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2451<F: Float>(t3070: F, t43198: F, t4578: F, t4574: F, t14192: F, t2960: F, t10510: F, t4641: F, t10316: F, t10481: F, t10483: F, t10877: F, t10952: F, t14099: F, t1616: F, t3071: F, t42347: F, t42511: F, t42743: F, t43176: F, t43291: F, t43292: F, t43385: F, t4579: F, t4582: F, t45872: F, t4593: F, t4600: F, t973: F, t974: F, t998: F) -> F {
    let t50147 = t3070 * t43198 * t4578;
    let t50148 = t50147 / F::new(6912.0);
    let t50169 = t3070 * t43198 * t4574;
    let t50170 = t50169 / F::new(6912.0);
    let t50172 = t2960 * t14192;
    let t50174 = t4641 * t10510;
    let t50175 = t50174 / F::new(4608.0);
    let t50176 = t973 * t974 * t998 * t45872 / F::new(288.0) + t3070 * t3071 * t1616 * t10316 / F::new(768.0) - t50148 + t42511 * t4579 / F::new(1536.0) + t43291 * t4582 * t4593 * t43292 * t10481 / F::new(128.0) - F::new(3.0) / F::new(256.0) * t43385 * t4582 * t4593 * t10483 + F::new(7.0) / F::new(1536.0) * t42347 * t4582 * t4593 * t10877 - t42743 * t4600 / F::new(1024.0) - t10952 * t14099 / F::new(512.0) - t50170 + t43176 / F::new(4608.0) - t50172 / F::new(54.0) - t50175;
    t50176
}
