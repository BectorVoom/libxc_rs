//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1222/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1222<F: Float>(t191: F, t192: F, t27215: F, t2020: F, t26142: F, t7042: F, t25010: F, t8607: F, t23938: F, t7468: F, t120067: F, t19456: F, t2040: F, t26878: F, t27150: F, t31057: F, t31060: F, t31726: F, t4028: F, t6517: F, t652: F, t7056: F, t7670: F, t8450: F, t8529: F, t90400: F) -> (F,) {
    let t121210 = t27215 * t191 * t192;
    let t121211 = t121210 * t2020;
    let t121224 = 2.0 * t7042 * t26142;
    let t121226 = t8607 * t25010;
    let t121228 = 2.0 * t23938 * t7468;
    let t121229 = -2.0 * t652 * t7056 * t7670 - 2.0 * t19456 * t8529 - 2.0 * t2040 * t90400 - t26878 * t8450 - 2.0 * t27150 * t6517 - 2.0 * t31726 * t4028 - t120067 + t121211 - t121224 - t121226 - t121228 - t31057 - t31060;
    (t121229,)
}
