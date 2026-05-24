//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1076/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1076<F: Float>(t27111: F, t3351: F, t515: F, t9188: F, t16156: F, t9184: F, t1001: F, t570: F, t9210: F, t26283: F, t26291: F, t29838: F, t36992: F, t36994: F, t36998: F, t37000: F, t37006: F, t41440: F, t41443: F, t41518: F, t42196: F, t42199: F, t42201: F, t42205: F, t42207: F, t42211: F) -> F {
    let t42215 = t3351 * t9188 * t515 * t27111;
    let t42217 = t16156 * t9184;
    let t42222 = t3351 * t9210 * t515 * t570 * t1001;
    let t42227 = -F::cast_from(0.14369090042236570277e1_f64) * t26283 * t41440 - F::cast_from(0.71845450211182851384e0_f64) * t26291 * t41443 + F::cast_from(0.95793933614910468512e0_f64) * t29838 * t41518 - F::cast_from(0.13334279030964389289e0_f64) * t42196 + F::new(2.0) * t36992 - F::cast_from(0.6818665413561335432e-1_f64) * t42199 - F::cast_from(0.72732431077987577943e-1_f64) * t42201 - F::new(0.4726e1) * t36994 + t42205 - t42207 + F::cast_from(0.25538759935978703638e-4_f64) * t42211 - F::cast_from(0.25538759935978703638e-4_f64) * t42215 + F::cast_from(0.59590439850616975156e-4_f64) * t42217 - F::cast_from(0.85129199786595678796e-5_f64) * t42222 + F::cast_from(0.79828278012425390426e-1_f64) * t36998 - F::cast_from(0.39914139006212695213e-1_f64) * t37000 + F::cast_from(0.47896966807455234256e0_f64) * t37006;
    t42227
}
