//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1422/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1422<F: Float>(t43713: F, t43717: F, t43721: F, t43725: F, t43727: F, t43729: F, t43734: F, t43737: F, t43740: F, t43743: F, t43746: F, t43748: F, t43750: F, t43754: F) -> F {
    let t44021 = -F::cast_from(0.98587999999999999999e0_f64) * t43713 - F::cast_from(0.10954222222222222222e0_f64) * t43717 + F::new(0.295764e1) * t43721 + F::cast_from(0.65725333333333333332e0_f64) * t43725 + F::cast_from(0.79724444444444444444e0_f64) * t43727 - F::cast_from(0.23917333333333333334e1_f64) * t43729 + F::cast_from(0.19931111111111111111e1_f64) * t43734 - F::cast_from(0.71752000000000000001e1_f64) * t43737 - F::cast_from(0.79724444444444444444e0_f64) * t43740 + F::new(0.107628e2) * t43743 + F::cast_from(0.23917333333333333333e1_f64) * t43746 - F::cast_from(0.5314962962962962963e0_f64) * t43748 - F::cast_from(0.44291358024691358024e0_f64) * t43750 - F::cast_from(0.82156666666666666668e-1_f64) * t43754;
    t44021
}
