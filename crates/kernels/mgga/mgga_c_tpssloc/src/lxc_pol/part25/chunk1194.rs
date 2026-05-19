//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1194/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1194<F: Float>(t80885: F, t80899: F, t80876: F, t80878: F, t80889: F, t80897: F, t80904: F, t80906: F, t80908: F, t80911: F, t80915: F, t80918: F, t80920: F, t80922: F, t80925: F, t80928: F, t80931: F, t80934: F, t80937: F, t80940: F) -> F {
    let t84533 = F::cast_from(0.67287926823567318088e-4_f64) * t80885;
    let t84536 = F::new(595.0) / F::new(2592.0) * t80899;
    let t84551 = -t80876 / F::new(64.0) - t80878 / F::new(192.0) - t84533 - F::cast_from(0.35608770875031824732e0_f64) * t80889 - F::cast_from(0.13565246047631171326e0_f64) * t80897 - t84536 - t80904 / F::new(128.0) + t80906 / F::new(128.0) + F::new(5.0) / F::new(64.0) * t80908 - t80911 / F::new(256.0) - F::new(119.0) / F::new(1152.0) * t80915 - F::cast_from(0.12111826828242117256e-2_f64) * t80918 + F::cast_from(0.84782787797694820791e-2_f64) * t80920 + F::cast_from(0.84782787797694820791e-2_f64) * t80922 - F::cast_from(0.40372756094140390853e-3_f64) * t80925 - F::cast_from(0.40372756094140390853e-3_f64) * t80928 + F::new(3.0) / F::new(8.0) * t80931 + F::cast_from(0.50869672678616892474e-1_f64) * t80934 + F::cast_from(0.24223653656484234512e-2_f64) * t80937 - F::cast_from(0.67826230238155856633e-1_f64) * t80940;
    t84551
}
