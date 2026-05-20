//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1359/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1359<F: Float>(t25146: F, t5614: F, t20949: F, t6621: F, t20947: F, t221: F, t25154: F, t20857: F, t6605: F, t9972: F, t105309: F, t105311: F, t105313: F, t105315: F, t105317: F, t105319: F, t105325: F, t105329: F, t105333: F, t105335: F, t105337: F, t81850: F, t81853: F, t98709: F, t98711: F, t98725: F) -> F {
    let t105339 = t25146 * t5614;
    let t105341 = t6621 * t20949;
    let t105345 = t25154 * t221 * t20947;
    let t105348 = t6605 * t9972 * t20857;
    let t105350 = -t105309 / F::new(512.0) + t105311 / F::new(256.0) - t105313 / F::new(128.0) - t105315 / F::new(384.0) - t105317 / F::new(128.0) + F::new(5.0) / F::new(128.0) * t105319 - F::new(7.0) / F::new(16.0) * t98709 - F::cast_from(0.17804385437515912366e0_f64) * t98711 - t81850 - t81853 - F::cast_from(0.60559134141210586281e-3_f64) * t105325 + F::cast_from(0.36335480484726351768e-2_f64) * t105329 + F::cast_from(0.12111826828242117256e-2_f64) * t105333 - t105335 / F::new(1536.0) - t105337 / F::new(512.0) - t105339 / F::new(512.0) + F::new(5.0) / F::new(128.0) * t105341 + F::cast_from(0.42391393898847410397e-2_f64) * t98725 + F::new(3.0) / F::new(16.0) * t105345 - F::cast_from(0.12111826828242117256e-2_f64) * t105348;
    t105350
}
