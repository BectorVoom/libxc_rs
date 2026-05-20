//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2350/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2350<F: Float>(t91486: F, t225: F, t26329: F, t26229: F, t81375: F, t1324: F, t254: F, t12020: F, t1386: F, t16439: F, t1843: F, t22656: F, t22670: F, t26224: F, t26226: F, t5210: F, t5325: F, t5326: F, t568: F, t6955: F, t6992: F, t6993: F, t80704: F) -> F {
    let t91487 = F::cast_from(0.16449340668482264365e-1_f64) * t91486;
    let t91488 = t26329 * t225;
    let t91491 = t26229 * t225;
    let t91496 = F::cast_from(0.25587863262083522346e0_f64) * t81375;
    let t91505 = t1324 * t254;
    let t91512 = -F::new(12.0) * t12020 * t26224 * t5325 * t6992 + F::new(2.0) * t5210 * t568 * t6955 - F::new(2.0) * t1386 * t91488 - F::new(2.0) * t1386 * t91491 - F::new(2.0) * t16439 * t6993 - t1843 * t80704 + F::new(4.0) * t22656 * t5326 + F::new(4.0) * t22670 * t5326 - F::new(12.0) * t26226 * t91505 + t91487 - t91496;
    t91512
}
