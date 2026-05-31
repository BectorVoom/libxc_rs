//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1961/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1961<F: Float>(t12461: F, t7939: F, t29376: F, t532: F, t193: F, t200: F, t7844: F, t1877: F, t2057: F, t24191: F, t25015: F, t25021: F, t2522: F, t25366: F, t25392: F, t26563: F, t26744: F, t28252: F, t7110: F, t7114: F, t92319: F, t97956: F, t97990: F, t98004: F, t98008: F, t98059: F, t98079: F, t98094: F, t99049: F, t99056: F) -> (F, F, F, F) {
    let t101138 = t7939 * t12461;
    let t101150 = t532 * t29376;
    let t101196 = t193 * t200 * t7844;
    let t101209 = -F::cast_from(3.0_f64) * t92319 * t25021 - F::cast_from(3.0_f64) * t24191 * t98079 - F::cast_from(3.0_f64) * t24191 * t99049 - F::cast_from(3.0_f64) * t24191 * t98008 + F::cast_from(3.0_f64) * t26563 * t99056 - t1877 * t7114 * t97990 - F::cast_from(6.0_f64) * t26563 * t98059 - F::cast_from(3.0_f64) * t92319 * t25366 - F::cast_from(3.0_f64) * t26563 * t97956 + F::cast_from(6.0_f64) * t101196 * t25015 - t1877 * t26744 * t25392 + F::cast_from(3.0_f64) * t24191 * t98004 + F::cast_from(3.0_f64) * t2522 * t7110 * t28252 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t98094;
    (t101138, t101150, t101196, t101209)
}
