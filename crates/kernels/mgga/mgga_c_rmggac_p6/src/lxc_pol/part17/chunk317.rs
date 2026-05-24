//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 317/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk317<F: Float>(t1767: F, t326: F, t128: F, t1743: F, t1763: F, t874: F, t118: F, t1756: F, t338: F, t1927: F, t1929: F, t1931: F, t1934: F, t1937: F) -> (F, F, F, F, F, F, F, F) {
    let t1939 = t326 * t1767;
    let t1941 = t128 * t1743;
    let t1942 = t326 * t1941;
    let t1944 = t874 * t1763;
    let t1945 = t118 * t1944;
    let t1947 = t338 * t1756;
    let t1948 = t118 * t1947;
    let t1950 = -F::cast_from(0.11974241701863808564e0_f64) * t1927 + F::cast_from(0.35922725105591425692e0_f64) * t1929 + F::cast_from(0.11974241701863808564e0_f64) * t1931 - F::cast_from(0.59871208509319042821e-1_f64) * t1934 - F::cast_from(0.23948483403727617128e0_f64) * t1937 - F::cast_from(0.11974241701863808564e0_f64) * t1939 + F::cast_from(0.59871208509319042821e-1_f64) * t1942 - F::cast_from(0.39914139006212695214e-1_f64) * t1945 + F::cast_from(0.19957069503106347607e-1_f64) * t1948;
    (t1939, t1941, t1942, t1944, t1945, t1947, t1948, t1950)
}
