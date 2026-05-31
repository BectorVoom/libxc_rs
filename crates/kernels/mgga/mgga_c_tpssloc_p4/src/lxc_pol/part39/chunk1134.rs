//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1134/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1134<F: Float>(t1022: F, t883: F, t607: F, t14211: F, t3071: F, t1615: F, t360: F, t4342: F, t1025: F, t10403: F, t1041: F, t10413: F, t10909: F, t10923: F, t10927: F, t14174: F, t14180: F, t14184: F, t14189: F, t14194: F, t14198: F, t14203: F, t14207: F, t2960: F, t3070: F, t3117: F, t4590: F, t4609: F, t973: F) -> (F, F) {
    let t14212 = t1022 * t883;
    let t14213 = t14212 * t607;
    let t14214 = t14211 * t14213;
    let t14215 = t3071 * t14214;
    let t14218 = t1615 * t1022;
    let t14219 = t360 * t883;
    let t14220 = t14219 * t607;
    let t14221 = t14218 * t14220;
    let t14222 = t3071 * t14221;
    let t14227 = t607 * t1022;
    let t14228 = t14227 * t360;
    let t14229 = t4342 * t14228;
    let t14230 = t3071 * t14229;
    let t14233 = -F::cast_from(5.0_f64) / F::cast_from(2304.0_f64) * t1041 * t14174 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t3117 * t4590 + F::cast_from(5.0_f64) / F::cast_from(6912.0_f64) * t1041 * t14180 + F::cast_from(5.0_f64) / F::cast_from(13824.0_f64) * t1041 * t14184 + F::cast_from(5.0_f64) / F::cast_from(5184.0_f64) * t1041 * t14189 + t14194 - t2960 * t4609 / F::cast_from(54.0_f64) + t973 * t14198 / F::cast_from(288.0_f64) - t14203 / F::cast_from(20736.0_f64) + t14207 * t1025 / F::cast_from(1536.0_f64) + t10909 / F::cast_from(4608.0_f64) + t10403 * t14215 / F::cast_from(1152.0_f64) - t10413 * t14222 / F::cast_from(2304.0_f64) - t10923 / F::cast_from(648.0_f64) - t10927 / F::cast_from(162.0_f64) - t3070 * t14230 / F::cast_from(1152.0_f64);
    (t14228, t14233)
}
