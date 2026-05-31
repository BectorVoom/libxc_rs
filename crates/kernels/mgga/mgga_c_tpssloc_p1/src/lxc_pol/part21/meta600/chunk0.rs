//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2353/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2353<F: Float>(t2509: F, t2512: F, t745: F, t9711: F, t1294: F, t2504: F, t9493: F, t2369: F, t9489: F, t116: F, t4: F, t126: F, t268: F, t8705: F) -> (F, F, F, F, F, F, F) {
    let t39259 = t2509 * t9711 * t2512 * t745;
    let t39261 = F::cast_from(0.69263436422725855036e2_f64) * t1294 * t39259;
    let t39263 = t9493 * t2504;
    let t39264 = t9489 * t2369 * t39263;
    let t39266 = F::cast_from(0.61524113149298439947e4_f64) * t1294 * t39264;
    let t39267 = t116 * t4;
    let t39273 = F::cast_from(1.0_f64) / t126 / t39267 * t116 * t8705 * t268 / F::cast_from(48.0_f64);
    (t39259, t39261, t39263, t39264, t39266, t39267, t39273)
}
