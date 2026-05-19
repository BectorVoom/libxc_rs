//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 625/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk625<F: Float>(t5828: F, t977: F, t3003: F, t4384: F, t5718: F, t5721: F, t5724: F, t340: F, t343: F, t974: F, t1597: F, t2969: F, t2986: F, t4507: F, t4529: F, t5818: F, t5821: F, t5825: F, t973: F) -> (F, F, F, F, F) {
    let t5829 = t977 * t5828;
    let t5836 = -t3003 - F::new(2.0) / F::new(9.0) * t4384 + t5718 / F::new(18.0) - t5721 / F::new(3.0) + t5724 / F::new(6.0);
    let t5837 = t340 * t5836;
    let t5838 = t5837 * t343;
    let t5839 = t974 * t5838;
    let t5842 = t1597 * t1597;
    let t5843 = t340 * t5842;
    let t5844 = t5843 * t343;
    let t5845 = t974 * t5844;
    let t5848 = -t2969 + F::cast_from(0.18518518518518518518e-3_f64) * t4507 - F::cast_from(0.55555555555555555554e-3_f64) * t4529 + F::cast_from(0.37037037037037037036e-3_f64) * t973 * t5818 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t5821 - F::cast_from(0.55555555555555555554e-3_f64) * t973 * t5825 + F::cast_from(0.27777777777777777777e-3_f64) * t973 * t5829 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t5839 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t5845;
    (t5836, t5838, t5842, t5844, t5848)
}
