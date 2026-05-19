//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 887/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk887<F: Float>(t164: F, t7886: F, t2275: F, t7850: F, t2319: F, t650: F, t209: F, t698: F, t2272: F, t713: F, t2250: F, t2255: F, t2258: F, t2267: F, t2268: F, t2273: F, t2276: F, t2314: F, t2321: F, t2324: F, t2327: F, t262: F, t699: F, t704: F, t706: F, t714: F, t721: F, t7814: F, t7844: F, t7849: F, t7853: F, t7858: F, t7859: F, t7871: F, t7876: F, t7879: F, t7882: F) -> F {
    let t7887 = t164 * t7886;
    let t7888 = t7850 * t2275;
    let t7895 = t650 * t2319;
    let t7899 = t209 * t698;
    let t7906 = t650 * t2272;
    let t7914 = t209 * t713;
    let t7921 = F::cast_from(0.35089341735807877242e1_f64) * t2327 * t7814 + F::new(1.0) * t699 * t7844 + F::cast_from(0.2069040516770936012e4_f64) * t7849 * t7853 - F::cast_from(0.10389515463408878255e3_f64) * t7858 * t7859 + F::cast_from(0.5848223622634646207e0_f64) * t714 * t7871 + F::cast_from(0.10254018858216406658e4_f64) * t7876 * t7879 + F::new(6.0) * t2273 * t7882 - F::cast_from(0.19298375398431042081e3_f64) * t7887 * t7888 + F::cast_from(0.96491876992155210402e2_f64) * t2273 * t2267 * t2275 * t704 + F::cast_from(0.32530743900905219526e-1_f64) * t262 * t7895 * t2321 + F::cast_from(0.68493333333333333332e-1_f64) * t262 * t7899 * t706 - F::cast_from(0.51369999999999999999e-1_f64) * t262 * t2250 * t2268 - F::cast_from(0.16522625736956710527e1_f64) * t262 * t7906 * t2276 + F::new(0.10274e0) * t262 * t650 * t2255 * t2258 + F::cast_from(0.21687162600603479684e-1_f64) * t262 * t7914 * t721 - F::cast_from(0.16265371950452609763e-1_f64) * t262 * t2314 * t2324;
    t7921
}
