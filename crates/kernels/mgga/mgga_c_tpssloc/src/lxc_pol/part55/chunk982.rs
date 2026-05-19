//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 982/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk982<F: Float>(t16439: F, t1843: F, t2016: F, t22656: F, t22907: F, t22909: F, t22921: F, t22924: F, t22926: F, t22928: F, t22940: F, t3758: F, t5215: F, t5321: F, t5354: F, t6958: F, t6963: F, t6993: F, t7729: F) -> F {
    let t26500 = F::cast_from(0.38381794893125283518e-1_f64) * t22907 + F::cast_from(0.19190897446562641759e-1_f64) * t22909 - t6958 * t5354 - t5321 * t6993 + F::new(2.0) * t5215 * t6963 - t22656 * t1843 + F::cast_from(0.82246703342411321824e-2_f64) * t22921 - t16439 * t2016 + t22924 + t22926 - F::cast_from(0.41123351671205660912e-2_f64) * t22928 + F::new(2.0) * t3758 * t7729 - t5215 * t6993 - F::cast_from(0.19190897446562641759e-1_f64) * t22940;
    t26500
}
