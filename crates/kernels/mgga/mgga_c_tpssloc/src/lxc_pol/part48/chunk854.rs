//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 854/910 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk854<F: Float>(t23035: F, t23241: F, t31366: F, t114790: F, t23164: F, t6555: F, t1880: F, t23237: F, t31419: F, t2047: F, t212: F, t23171: F, t6554: F, t112915: F, t112920: F, t112927: F, t112932: F, t112936: F, t112942: F, t2053: F, t22978: F, t23190: F, t23278: F, t23281: F, t24305: F, t25168: F, t26728: F, t2713: F, t2718: F, t31400: F, t6632: F, t7092: F, t7107: F, t855: F) -> (F,) {
    let t114913 = t23035 * t31366 * t23241;
    let t114916 = t23164 * t114790 * t6555;
    let t114926 = t1880 * t23237 * t31419;
    let t114932 = t23171 * t212 * t2047 * t6554;
    let t114933 = 0.82246703342411321824e-2 * t114932;
    let t114934 = 2.0 * t855 * t2718 * t2053 * t23190 + 4.0 * t23281 * t7092 - t112915 + 4.0 * t24305 * t6632 - t112920 + 0.49348022005446793095e-1 * t114913 + t112927 - t112932 + 0.16449340668482264365e-1 * t114916 - 2.0 * t23281 * t7107 - 12.0 * t25168 * t26728 * t22978 - 2.0 * t2713 * t31400 - 0.16449340668482264365e-1 * t114926 + t112936 + 4.0 * t23278 * t7092 - t114933 - t112942;
    (t114934,)
}
