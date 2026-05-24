//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1108/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1108<F: Float>(t1543: F, t2993: F, t2975: F, t4184: F, t3001: F, t4180: F, t1080: F, t12177: F, t12180: F, t12183: F, t12187: F, t12190: F, t12194: F, t12201: F, t2930: F, t2955: F, t2974: F, t2999: F, t4163: F, t4185: F, t9359: F, t9370: F, t9373: F, t9424: F, t9465: F) -> F {
    let t12204 = t1543 * t2993;
    let t12207 = t4184 * t2975;
    let t12210 = t4180 * t3001;
    let t12211 = t12210 * t1080;
    let t12214 = t4184 * t2993;
    let t12217 = -F::new(4.0) * t2930 * t12177 - F::new(2.0) * t2930 * t12180 - F::cast_from(0.19298375398431042081e3_f64) * t9424 * t12183 + F::cast_from(0.64327917994770140268e2_f64) * t2955 * t12187 + F::cast_from(0.32163958997385070134e2_f64) * t2955 * t12190 + F::cast_from(0.2069040516770936012e4_f64) * t9465 * t12194 - F::cast_from(0.23392894490538584828e1_f64) * t9359 * t4163 + F::cast_from(0.34631718211362927518e2_f64) * t9370 * t4185 - F::cast_from(0.23392894490538584828e1_f64) * t2974 * t12201 - F::cast_from(0.11696447245269292414e1_f64) * t2974 * t12204 - F::cast_from(0.10389515463408878255e3_f64) * t9373 * t12207 + F::cast_from(0.34631718211362927518e2_f64) * t2999 * t12211 + F::cast_from(0.17315859105681463759e2_f64) * t2999 * t12214;
    t12217
}
