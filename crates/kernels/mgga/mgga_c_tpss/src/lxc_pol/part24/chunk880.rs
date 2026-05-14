//! MGGA_C_TPSS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 880/1347 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part24_v4rho3sigma_6_chunk880<F: Float>(t209: F, t698: F, t2272: F, t650: F, t713: F, t2250: F, t2255: F, t2258: F, t2267: F, t2268: F, t2273: F, t2275: F, t2276: F, t2314: F, t2321: F, t2324: F, t2327: F, t262: F, t699: F, t704: F, t706: F, t714: F, t721: F, t7814: F, t7844: F, t7849: F, t7853: F, t7858: F, t7859: F, t7871: F, t7876: F, t7879: F, t7882: F, t7887: F, t7888: F, t7895: F) -> (F,) {
    let t7899 = t209 * t698;
    let t7906 = t650 * t2272;
    let t7914 = t209 * t713;
    let t7921 = 0.35089341735807877242e1 * t2327 * t7814 + 1.0 * t699 * t7844 + 0.2069040516770936012e4 * t7849 * t7853 - 0.10389515463408878255e3 * t7858 * t7859 + 0.5848223622634646207e0 * t714 * t7871 + 0.10254018858216406658e4 * t7876 * t7879 + 6.0 * t2273 * t7882 - 0.19298375398431042081e3 * t7887 * t7888 + 0.96491876992155210402e2 * t2273 * t2267 * t2275 * t704 + 0.32530743900905219526e-1 * t262 * t7895 * t2321 + 0.68493333333333333332e-1 * t262 * t7899 * t706 - 0.51369999999999999999e-1 * t262 * t2250 * t2268 - 0.16522625736956710527e1 * t262 * t7906 * t2276 + 0.10274e0 * t262 * t650 * t2255 * t2258 + 0.21687162600603479684e-1 * t262 * t7914 * t721 - 0.16265371950452609763e-1 * t262 * t2314 * t2324;
    (t7921,)
}
